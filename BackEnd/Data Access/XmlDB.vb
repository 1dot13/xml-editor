Imports System.Data

Public Class XmlDB
    Implements IDisposable

    Protected ds As DataSet

    Public Event BeforeLoadAll(sender As XmlDB)
    Public Event AfterLoadAll(sender As XmlDB)
    Public Event BeforeSaveAll(sender As XmlDB)
    Public Event AfterSaveAll(sender As XmlDB)
    Public Event LoadingTable(sender As XmlDB, ByVal fileName As String)
    Public Event SavingTable(sender As XmlDB, ByVal fileName As String)

    Public Event BeforeLoadTable(sender As XmlDB, ByVal table As DataTable)
    Public Event AfterLoadTable(sender As XmlDB, ByVal table As DataTable)
    Public Event BeforeSaveTable(sender As XmlDB, ByVal table As DataTable)
    Public Event AfterSaveTable(sender As XmlDB, ByVal table As DataTable)

    Protected _schemaName As String
    Protected _schemaFileName As String
    Protected _dataManager As DataManager

    Public Sub New(schemaName As String, schemaFileName As String, dataMgr As DataManager)
        If String.IsNullOrEmpty(schemaName) Then Throw New ArgumentNullException("Missing schema name argument.")
        If dataMgr Is Nothing Then Throw New ArgumentNullException("Missing data manager argument.")

        _schemaName = schemaName
        _schemaFileName = schemaFileName
        _dataManager = dataMgr
    End Sub

    Public ReadOnly Property SchemaName As String
        Get
            Return _schemaName
        End Get
    End Property

    Public ReadOnly Property SchemaFileName As String
        Get
            Return _schemaFileName
        End Get
    End Property

    Public ReadOnly Property DataManager As DataManager
        Get
            Return _dataManager
        End Get
    End Property

    Public Property DataSet() As DataSet
        Get
            Return ds
        End Get
        Set(value As DataSet)
            ds = value
            InitializeHandlers()
        End Set
    End Property

    Public Sub InitializeHandlers()
        For Each t As DataTable In ds.Tables
            Dim handler As DefaultTable
            Dim handlerName As String = t.GetStringProperty(TableProperty.TableHandlerName)
            If handlerName Is Nothing Then
                handler = New DefaultTable(t, _dataManager)
            Else
                Dim obj As Object = Activator.CreateInstance(Type.GetType("BackEnd." & handlerName), t, _dataManager)
                handler = DirectCast(obj, DefaultTable)
            End If
            If t.ExtendedProperties.Contains(TableProperty.TableHandler) Then t.ExtendedProperties.Remove(TableProperty.TableHandler)
            t.ExtendedProperties.Add(TableProperty.TableHandler, handler)
        Next
    End Sub

    Public Sub RemoveHandlers()
        For Each t As DataTable In ds.Tables
            If t.ExtendedProperties.Contains(TableProperty.TableHandler) Then
                Dim handler As DefaultTable = t.ExtendedProperties(TableProperty.TableHandler)
                handler.Dispose()
                t.ExtendedProperties.Remove(TableProperty.TableHandler)
            End If
        Next
    End Sub

    Public ReadOnly Property Table(ByVal tableName As String) As DataTable
        Get
            If ds.Tables.Contains(tableName) Then
                Return ds.Tables(tableName)
            End If
            Return Nothing
        End Get
    End Property

    Public Sub SaveSchema()
        ds.WriteXmlSchema(ds.DataSetName & ".xsd")
    End Sub

    Public Sub LoadSchema(ByVal schemaFileName As String)
        If ds IsNot Nothing Then
            ds.Clear()
            ds.Tables.Clear()
            ds.Relations.Clear()
            ds.Dispose()
        End If
        ds = New DataSet
        ds.ReadXmlSchema(schemaFileName)
        For Each t As DataTable In ds.Tables
            Dim handler As DefaultTable
            Dim handlerName As String = t.GetStringProperty(TableProperty.TableHandlerName)
            If handlerName Is Nothing Then
                handler = New DefaultTable(t, _dataManager)
            Else
                Dim obj As Object = Activator.CreateInstance(Type.GetType("BackEnd." & handlerName), t, _dataManager)
                handler = DirectCast(obj, DefaultTable)
            End If
            If t.ExtendedProperties.Contains(TableProperty.TableHandler) Then t.ExtendedProperties.Remove(TableProperty.TableHandler)
            t.ExtendedProperties.Add(TableProperty.TableHandler, handler)
        Next
    End Sub

    Public Sub LoadProperties(schemaFileName As String)
        Dim tempDS As New DataSet
        tempDS.ReadXmlSchema(schemaFileName)

        For Each t As DataTable In ds.Tables
            If tempDS.Tables.Contains(t.TableName) Then
                Dim tempTable As DataTable = tempDS.Tables(t.TableName)
                For Each c As DataColumn In t.Columns
                    If tempTable.Columns.Contains(c.ColumnName) Then
                        Dim tempCol As DataColumn = tempTable.Columns(c.ColumnName)
                        c.SetProperty(ColumnProperty.Width, tempCol.GetStringProperty(ColumnProperty.Width))
                        c.SetProperty(ColumnProperty.Grid_Hidden, tempCol.GetBooleanProperty(ColumnProperty.Grid_Hidden))

                        'add more here later if required
                    End If
                Next

            End If
        Next
    End Sub

    Public Function GetHandler(ByVal tableName As String) As DefaultTable
        If ds.Tables.Contains(tableName) Then
            Return GetTableHandler(ds.Tables(tableName))
        End If
        Return Nothing
    End Function

    Protected Sub BeginInit()
        ds.BeginInit()
        ds.EnforceConstraints = False
    End Sub

    Protected Sub EndInit()
        ds.AcceptChanges()

        ' Proactive validation before enforcing constraints
        Dim validator As New XmlValidator(ds)
        If Not validator.ValidateAll() Then
            ' Show detailed validation errors
            Dim errorMessage As String = validator.GetFormattedErrors()
            ErrorHandler.ShowError(errorMessage, "XML Validation Failed", MessageBoxIcon.Error)
            ErrorHandler.TriggerFatalError()
            Return
        End If

        Try
            ds.EnforceConstraints = True
        Catch ex As ConstraintException
            ' This is now a fallback - should rarely be hit due to proactive validation
            Dim errMsg As New Text.StringBuilder()
            errMsg.AppendLine("Constraint validation failed after proactive checks.")
            errMsg.AppendLine("This may indicate a data race condition or complex constraint issue.")
            errMsg.AppendLine()
            errMsg.AppendLine("Error Details:")
            errMsg.AppendLine(ex.Message)
            errMsg.AppendLine()

            For Each t As DataTable In ds.Tables
                If t.HasErrors Then
                    Dim fileName As String = t.GetStringProperty(TableProperty.FileName)
                    If Not String.IsNullOrEmpty(fileName) Then
                        errMsg.AppendLine("File: " & fileName)
                    Else
                        errMsg.AppendLine("Table: " & t.TableName)
                    End If

                    ' Enhanced error reporting with row numbers and column info
                    Dim errorRows As DataRow() = t.GetErrors()
                    For i As Integer = 0 To Math.Min(errorRows.GetUpperBound(0), 9) ' Limit to 10 errors
                        Dim row As DataRow = errorRows(i)
                        Dim rowIndex As Integer = t.Rows.IndexOf(row)
                        errMsg.AppendLine($"  Row {rowIndex + 1}: {row.RowError}")

                        ' Show which columns have errors
                        For Each col As DataColumn In t.Columns
                            If Not String.IsNullOrEmpty(row.GetColumnError(col)) Then
                                errMsg.AppendLine($"    Column '{col.ColumnName}': {row.GetColumnError(col)}")
                            End If
                        Next
                    Next

                    If errorRows.Length > 10 Then
                        errMsg.AppendLine($"  ... and {errorRows.Length - 10} more error(s)")
                    End If
                    errMsg.AppendLine()
                End If
            Next

            ErrorHandler.ShowError(errMsg.ToString(), "Error Loading Files", MessageBoxIcon.Error, ex)
            ErrorHandler.TriggerFatalError()
        End Try
        ds.EndInit()
    End Sub

    Public Overridable Sub LoadAllData()
        RaiseEvent BeforeLoadAll(Me)
        ds.Clear()
        BeginInit()
        For Each t As DataTable In ds.Tables
            LoadData(t)
        Next
        EndInit()
        RaiseEvent AfterLoadAll(Me)
    End Sub

    Public Overridable Sub LoadData(ByVal tableName As String)
        Dim t As DataTable = ds.Tables(tableName)
        If Not t Is Nothing Then LoadData(t)
    End Sub

    Public Overridable Sub LoadData(ByVal table As DataTable)
        Application.DoEvents()
        RaiseEvent BeforeLoadTable(Me, table)
        Dim fileName As String = table.GetStringProperty(TableProperty.FileName)
        If fileName IsNot Nothing Then

            Dim loadData As Boolean = True

            ' RoWa21: If we have a language specifix XML file, load from specific location if file exists
            If IsLanguageSpecificFile(fileName) Then
                Dim filePath As String = _dataManager.GetLanguageSpecificTableDirectory(fileName)
                If System.IO.File.Exists(filePath & fileName) = False Then
                    loadData = False
                End If
            End If

            If loadData Then
                Try
                    RaiseEvent LoadingTable(Me, fileName)
                    table.GetTableHandler.LoadData()
                Catch ex As System.IO.FileNotFoundException
                    ErrorHandler.ShowError($"Failed to load '{fileName}': File not found.{vbCrLf}{vbCrLf}Path: {ex.FileName}", "File Not Found", MessageBoxIcon.Error, ex)
                    Throw
                Catch ex As Xml.XmlException
                    ErrorHandler.ShowError($"Failed to load '{fileName}': Invalid XML format.{vbCrLf}{vbCrLf}Line {ex.LineNumber}, Position {ex.LinePosition}: {ex.Message}", "XML Format Error", MessageBoxIcon.Error, ex)
                    Throw
                Catch ex As Exception
                    ErrorHandler.ShowError($"Failed to load '{fileName}': {ex.Message}{vbCrLf}{vbCrLf}Table: {table.TableName}", "Error Loading Table", MessageBoxIcon.Error, ex)
                    Throw
                End Try
            End If

        End If
        RaiseEvent AfterLoadTable(Me, table)
    End Sub

    Public Overridable Sub SaveAllData()
        RaiseEvent BeforeSaveAll(Me)
        For Each t As DataTable In ds.Tables
            SaveData(t)
        Next
        RaiseEvent AfterSaveAll(Me)
    End Sub
    Public Overridable Sub SaveData(ByVal tableName As String)
        Dim t As DataTable = ds.Tables(tableName)
        If Not t Is Nothing Then
            SaveData(t)
        End If
    End Sub

    Public Overridable Sub SaveData(ByVal table As DataTable)
        Application.DoEvents()
        RaiseEvent BeforeSaveTable(Me, table)
        Dim fileName As String = table.GetStringProperty(TableProperty.FileName)
        If fileName IsNot Nothing Then

            Dim writeData As Boolean = True

            Dim filePath = _dataManager.TableDirectory & fileName

            If IsLanguageSpecificFile(fileName) Then
                filePath = _dataManager.GetLanguageSpecificTableDirectory(fileName) & fileName

                If Not System.IO.File.Exists(filePath) Then
                    writeData = False
                End If
            End If

            If writeData Then
                RaiseEvent SavingTable(Me, fileName)
                table.GetTableHandler.SaveData()
            End If
        End If

        RaiseEvent AfterSaveTable(Me, table)
    End Sub

    Public Overridable Sub LoadWorkingData(fileName As String)
        RemoveHandlers()
        ds = New DataSet
        BeginInit()
        ds.ReadXml(fileName, XmlReadMode.ReadSchema)
        EndInit()
        InitializeHandlers()
    End Sub

    Public Overridable Sub SaveWorkingData(fileName As String)
        ds.WriteXml(fileName, XmlWriteMode.WriteSchema)
        ds.AcceptChanges()
    End Sub

    'these just work on single pk tables for now
    Public Function GetNextPrimaryKeyValue(ByVal tableName As String) As Decimal
        Return GetNextPrimaryKeyValue(ds.Tables(tableName))
    End Function
    Public Function GetNextPrimaryKeyValue(ByVal table As DataTable) As Decimal
        Return table.GetTableHandler.GetNextPrimaryKeyValue
    End Function

    Public Function NewRow(ByVal tableName As String) As DataRow
        Return NewRow(ds.Tables(tableName))
    End Function
    Public Function NewRow(ByVal table As DataTable) As DataRow
        Return table.GetTableHandler.NewRow
    End Function

    Public Sub DeleteRow(ByVal tableName As String, ByVal key As Decimal)
        DeleteRow(ds.Tables(tableName), key)
    End Sub
    Public Sub DeleteRow(ByVal table As DataTable, ByVal key As Decimal)
        table.GetTableHandler.DeleteRow(key)
    End Sub

    Public Function DuplicateRow(ByVal tableName As String, ByVal key As Decimal) As DataRow
        Return DuplicateRow(ds.Tables(tableName), key)
    End Function
    Public Function DuplicateRow(ByVal table As DataTable, ByVal key As Decimal) As DataRow
        Return table.GetTableHandler.DuplicateRow(key)
    End Function

    Public Function CopyRows(tableName As String, rows As DataRow(), source As XmlDB, Optional allowOverwrite As Boolean = True) As DataRow
        Return CopyRows(ds.Tables(tableName), rows, source, allowOverwrite)
    End Function

    Public Function CopyRows(table As DataTable, rows As DataRow(), source As XmlDB, Optional allowOverwrite As Boolean = True) As DataRow
        Return table.GetTableHandler.CopyRows(rows, source, allowOverwrite)
    End Function

#Region "IDisposable Support"
    Private _disposedValue As Boolean ' To detect redundant calls

    ' IDisposable
    Protected Overridable Sub Dispose(disposing As Boolean)
        If Not _disposedValue Then
            If disposing Then
                ' TODO: dispose managed state (managed objects).
                ds.Dispose()
            End If

            ' TODO: free unmanaged resources (unmanaged objects) and override Finalize() below.
            ' TODO: set large fields to null.
        End If
        _disposedValue = True
    End Sub

    ' TODO: override Finalize() only if Dispose(ByVal disposing As Boolean) above has code to free unmanaged resources.
    'Protected Overrides Sub Finalize()
    '    ' Do not change this code.  Put cleanup code in Dispose(ByVal disposing As Boolean) above.
    '    Dispose(False)
    '    MyBase.Finalize()
    'End Sub

    ' This code added by Visual Basic to correctly implement the disposable pattern.
    Public Sub Dispose() Implements IDisposable.Dispose
        ' Do not change this code.  Put cleanup code in Dispose(ByVal disposing As Boolean) above.
        Dispose(True)
        GC.SuppressFinalize(Me)
    End Sub
#End Region

End Class
