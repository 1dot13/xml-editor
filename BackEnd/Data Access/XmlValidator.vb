Imports System.Data
Imports System.Text

Public Class XmlValidator
    Private _dataSet As DataSet
    Private _validationErrors As New List(Of ValidationError)

    Public Sub New(dataSet As DataSet)
        _dataSet = dataSet
    End Sub

    Public ReadOnly Property HasErrors As Boolean
        Get
            Return _validationErrors.Count > 0
        End Get
    End Property

    Public ReadOnly Property Errors As List(Of ValidationError)
        Get
            Return _validationErrors
        End Get
    End ReadOnly Property

    ''' <summary>
    ''' Validates all data in the dataset before constraints are enforced
    ''' </summary>
    Public Function ValidateAll() As Boolean
        _validationErrors.Clear()

        ' Check for null/empty required fields
        ValidateRequiredFields()

        ' Check for duplicate primary keys
        ValidatePrimaryKeys()

        ' Check for foreign key violations
        ValidateForeignKeys()

        ' Check for unique constraint violations
        ValidateUniqueConstraints()

        Return Not HasErrors
    End Function

    ''' <summary>
    ''' Validates a single table before loading
    ''' </summary>
    Public Function ValidateTable(table As DataTable) As Boolean
        Dim startErrorCount = _validationErrors.Count

        ValidateTableRequiredFields(table)
        ValidateTablePrimaryKeys(table)
        ValidateTableForeignKeys(table)
        ValidateTableUniqueConstraints(table)

        Return _validationErrors.Count = startErrorCount
    End Function

    Private Sub ValidateRequiredFields()
        For Each table As DataTable In _dataSet.Tables
            ValidateTableRequiredFields(table)
        Next
    End Sub

    Private Sub ValidateTableRequiredFields(table As DataTable)
        For Each col As DataColumn In table.Columns
            If Not col.AllowDBNull AndAlso Not col.AutoIncrement Then
                For i As Integer = 0 To table.Rows.Count - 1
                    Dim row As DataRow = table.Rows(i)
                    If row.RowState <> DataRowState.Deleted AndAlso IsDBNull(row(col)) Then
                        AddError(table, i, col.ColumnName,
                                "Required field is empty",
                                ValidationErrorType.RequiredField)
                    End If
                Next
            End If
        Next
    End Sub

    Private Sub ValidatePrimaryKeys()
        For Each table As DataTable In _dataSet.Tables
            ValidateTablePrimaryKeys(table)
        Next
    End Sub

    Private Sub ValidateTablePrimaryKeys(table As DataTable)
        If table.PrimaryKey.Length = 0 Then Return

        Dim pkValues As New Dictionary(Of String, Integer)
        Dim pkColumns As String = String.Join(", ", table.PrimaryKey.Select(Function(c) c.ColumnName))

        For i As Integer = 0 To table.Rows.Count - 1
            Dim row As DataRow = table.Rows(i)
            If row.RowState = DataRowState.Deleted Then Continue For

            Dim keyValue As String = GetPrimaryKeyValue(row)
            If pkValues.ContainsKey(keyValue) Then
                AddError(table, i, pkColumns,
                        $"Duplicate primary key value: {keyValue}. First occurrence at row {pkValues(keyValue) + 1}",
                        ValidationErrorType.DuplicatePrimaryKey)
            Else
                pkValues.Add(keyValue, i)
            End If
        Next
    End Sub

    Private Sub ValidateForeignKeys()
        For Each table As DataTable In _dataSet.Tables
            ValidateTableForeignKeys(table)
        Next
    End Sub

    Private Sub ValidateTableForeignKeys(table As DataTable)
        For Each fk As ForeignKeyConstraint In table.Constraints.OfType(Of ForeignKeyConstraint)()
            Dim parentTable As DataTable = fk.RelatedTable
            Dim childColumns As String = String.Join(", ", fk.Columns.Select(Function(c) c.ColumnName))
            Dim parentColumns As String = String.Join(", ", fk.RelatedColumns.Select(Function(c) c.ColumnName))

            For i As Integer = 0 To table.Rows.Count - 1
                Dim row As DataRow = table.Rows(i)
                If row.RowState = DataRowState.Deleted Then Continue For

                ' Build foreign key value
                Dim fkValues(fk.Columns.Length - 1) As Object
                For j As Integer = 0 To fk.Columns.Length - 1
                    fkValues(j) = row(fk.Columns(j))
                Next

                ' Check if the value exists in parent table
                Dim parentRow As DataRow = parentTable.Rows.Find(fkValues)
                If parentRow Is Nothing AndAlso Not IsNullForeignKey(fkValues) Then
                    Dim fkValueStr As String = String.Join(", ", fkValues.Select(Function(v) If(v Is Nothing, "NULL", v.ToString())))
                    AddError(table, i, childColumns,
                            $"Foreign key violation: Value '{fkValueStr}' does not exist in table '{parentTable.TableName}' column(s) '{parentColumns}'",
                            ValidationErrorType.ForeignKeyViolation)
                End If
            Next
        Next
    End Sub

    Private Sub ValidateUniqueConstraints()
        For Each table As DataTable In _dataSet.Tables
            ValidateTableUniqueConstraints(table)
        Next
    End Sub

    Private Sub ValidateTableUniqueConstraints(table As DataTable)
        For Each uc As UniqueConstraint In table.Constraints.OfType(Of UniqueConstraint)()
            If uc.IsPrimaryKey Then Continue For ' Already checked

            Dim uniqueValues As New Dictionary(Of String, Integer)
            Dim columnNames As String = String.Join(", ", uc.Columns.Select(Function(c) c.ColumnName))

            For i As Integer = 0 To table.Rows.Count - 1
                Dim row As DataRow = table.Rows(i)
                If row.RowState = DataRowState.Deleted Then Continue For

                Dim keyValue As String = GetUniqueConstraintValue(row, uc.Columns)
                If uniqueValues.ContainsKey(keyValue) Then
                    AddError(table, i, columnNames,
                            $"Unique constraint violation: Duplicate value '{keyValue}'. First occurrence at row {uniqueValues(keyValue) + 1}",
                            ValidationErrorType.UniqueConstraintViolation)
                Else
                    uniqueValues.Add(keyValue, i)
                End If
            Next
        Next
    End Sub

    Private Function GetPrimaryKeyValue(row As DataRow) As String
        Dim values As New List(Of String)
        For Each col As DataColumn In row.Table.PrimaryKey
            values.Add(If(row(col) Is Nothing, "NULL", row(col).ToString()))
        Next
        Return String.Join("|", values)
    End Function

    Private Function GetUniqueConstraintValue(row As DataRow, columns As DataColumn()) As String
        Dim values As New List(Of String)
        For Each col As DataColumn In columns
            values.Add(If(row(col) Is Nothing, "NULL", row(col).ToString()))
        Next
        Return String.Join("|", values)
    End Function

    Private Function IsNullForeignKey(values As Object()) As Boolean
        For Each v In values
            If v Is Nothing OrElse IsDBNull(v) Then Return True
        Next
        Return False
    End Function

    Private Sub AddError(table As DataTable, rowIndex As Integer, columnName As String, message As String, errorType As ValidationErrorType)
        Dim fileName As String = table.GetStringProperty(TableProperty.FileName)
        If String.IsNullOrEmpty(fileName) Then fileName = table.TableName

        _validationErrors.Add(New ValidationError() With {
            .FileName = fileName,
            .TableName = table.TableName,
            .RowIndex = rowIndex,
            .ColumnName = columnName,
            .Message = message,
            .ErrorType = errorType
        })
    End Sub

    Public Function GetFormattedErrors() As String
        If Not HasErrors Then Return String.Empty

        Dim sb As New StringBuilder()
        sb.AppendLine("XML Validation Errors Found:")
        sb.AppendLine()

        ' Group errors by file
        Dim errorsByFile = _validationErrors.GroupBy(Function(e) e.FileName)

        For Each fileGroup In errorsByFile
            sb.AppendLine($"File: {fileGroup.Key}")
            sb.AppendLine(New String("-"c, 60))

            ' Group by error type
            Dim errorsByType = fileGroup.GroupBy(Function(e) e.ErrorType)

            For Each typeGroup In errorsByType
                sb.AppendLine($"  {GetErrorTypeDescription(typeGroup.Key)} ({typeGroup.Count()} error(s)):")

                Dim maxErrors As Integer = 10 ' Limit display to first 10 errors per type
                Dim displayCount As Integer = Math.Min(typeGroup.Count(), maxErrors)

                For i As Integer = 0 To displayCount - 1
                    Dim err = typeGroup.ElementAt(i)
                    sb.AppendLine($"    Row {err.RowIndex + 1}, Column '{err.ColumnName}': {err.Message}")
                Next

                If typeGroup.Count() > maxErrors Then
                    sb.AppendLine($"    ... and {typeGroup.Count() - maxErrors} more error(s)")
                End If
                sb.AppendLine()
            Next
        Next

        Return sb.ToString()
    End Function

    Private Function GetErrorTypeDescription(errorType As ValidationErrorType) As String
        Select Case errorType
            Case ValidationErrorType.RequiredField
                Return "Required Field Errors"
            Case ValidationErrorType.DuplicatePrimaryKey
                Return "Duplicate Primary Key Errors"
            Case ValidationErrorType.ForeignKeyViolation
                Return "Foreign Key Violations"
            Case ValidationErrorType.UniqueConstraintViolation
                Return "Unique Constraint Violations"
            Case Else
                Return "Other Errors"
        End Select
    End Function
End Class

Public Class ValidationError
    Public Property FileName As String
    Public Property TableName As String
    Public Property RowIndex As Integer
    Public Property ColumnName As String
    Public Property Message As String
    Public Property ErrorType As ValidationErrorType
End Class

Public Enum ValidationErrorType
    RequiredField
    DuplicatePrimaryKey
    ForeignKeyViolation
    UniqueConstraintViolation
    Other
End Enum
