Imports System.Xml
Imports System.IO
Imports System.Data

Public Class ControlTable
    Inherits DefaultTable

    Public Sub New(table As DataTable, manager As DataManager)
        MyBase.New(table, manager)
    End Sub


    Public Overrides Sub LoadData()
        Const Temp As String = "temp"
        Dim t As DataTable = Nothing

        _table.BeginLoadData()
        _table.Clear()
        Dim fileName As String = _table.GetStringProperty(TableProperty.FileName)
        Dim filePath As String = GetFilePath(fileName)

        Dim sourceName As String = _table.GetStringProperty(TableProperty.SourceTableName)

        If sourceName Is Nothing Then
        Else
            Dim tableName As String = _table.TableName
            For Each t In _table.DataSet.Tables
                If t.TableName = sourceName Then
                    t.TableName = Temp
                    Exit For
                End If
            Next

            _table.TableName = sourceName
            LoadControlData(fileName, filePath)

            _table.TableName = tableName
            If t IsNot Nothing AndAlso t.TableName = Temp Then t.TableName = sourceName
        End If
        _table.EndLoadData()
    End Sub

    Protected Overridable Sub LoadControlData(ByVal fileName As String, ByVal filePath As String)
        Dim xmldoc As New XmlDocument()
        Dim fs As New FileStream(filePath, FileMode.Open, FileAccess.Read)
        xmldoc.Load(fs)

        Dim xmlnode As XmlNode = xmldoc.GetElementsByTagName("INVENTORYLIST").Item(0)
        Dim xmlParentNode As XmlNodeList
        Dim xmlChildNode As XmlNodeList
        Dim xmlChild2Node As XmlNodeList

        For i As Integer = 0 To xmlnode.ChildNodes.Count - 1
            If xmlnode.ChildNodes.Item(i).Name = "INVENTORY" Then Continue For

            _table.Rows.Add()
            Dim rowIndex As Integer = _table.Rows.Count - 1
            _table.Rows(rowIndex).BeginEdit()

            xmlParentNode = xmlnode.ChildNodes.Item(i).ChildNodes
            For x As Integer = 0 To xmlParentNode.Count - 1
                Dim xmlElement As XmlNode = xmlParentNode.Item(x)
                Dim xmlElementName As String = xmlElement.Name

                If xmlElementName = "#comment" Then Continue For
                If xmlElementName = "REORDERDAYSDELAY" OrElse xmlElementName = "CASH" OrElse xmlElementName = "COOLNESS" OrElse xmlElementName = "BASICDEALERFLAGS" Then
                    xmlChildNode = xmlElement.ChildNodes

                    For y As Integer = 0 To xmlChildNode.Count - 1
                        Dim xmlChildElement As XmlNode = xmlChildNode.Item(y)
                        Dim xmlChildName As String = xmlChildElement.Name

                        If xmlChildName = "#comment" Then Continue For
                        If xmlChildName = "DAILY" Then
                            xmlChild2Node = xmlChildElement.ChildNodes

                            For z As Integer = 0 To xmlChild2Node.Count - 1
                                Dim xmlChild2Element As XmlNode = xmlChild2Node.Item(z)
                                Dim xmlChild2Name As String = xmlChild2Element.Name

                                If _table.Columns.Contains(xmlChild2Name) Then
                                    If xmlChild2Name = "#comment" Then Continue For
                                    If _table.Columns(xmlChild2Name).DataType.Name = "Boolean" Then
                                        _table.Rows(rowIndex).Item(xmlChild2Name) = IIf(xmlChild2Element.InnerText.Trim = 1, True, False)
                                    Else
                                        _table.Rows(rowIndex).Item(xmlChild2Name) = xmlChild2Element.InnerText.Trim
                                    End If
                                End If
                            Next
                        Else
                            If _table.Columns.Contains(xmlChildName) Then
                                If _table.Columns(xmlChildName).DataType.Name = "Boolean" Then
                                    _table.Rows(rowIndex).Item(xmlChildName) = IIf(xmlChildElement.InnerText.Trim = 1, True, False)
                                Else
                                    _table.Rows(rowIndex).Item(xmlChildName) = xmlChildElement.InnerText.Trim
                                End If
                            End If
                        End If
                    Next
                Else
                    If _table.Columns.Contains(xmlElementName) Then
                        If _table.Columns(xmlElementName).DataType.Name = "Boolean" Then
                            _table.Rows(rowIndex).Item(xmlElementName) = IIf(xmlElement.InnerText.Trim = 1, True, False)
                        Else
                            _table.Rows(rowIndex).Item(xmlElementName) = xmlElement.InnerText.Trim
                        End If
                    End If
                End If
            Next
            _table.Rows(rowIndex).EndEdit()
        Next

        fs.Close()
        fs.Dispose()
    End Sub


    Protected Overrides Sub WriteXml(ByVal table As DataTable, ByVal fileName As String)
        Dim view As New DataView(table, "", table.Columns(0).ColumnName, DataViewRowState.CurrentRows)
        Dim xw As New XmlTextWriter(fileName, System.Text.Encoding.UTF8)
        Dim i As Integer
        Dim x As Integer
        Dim trim As Boolean = True
        Dim child1 As Boolean = False
        Dim child2 As Boolean = False

        xw.WriteStartDocument()
        xw.WriteWhitespace(vbLf)
        xw.WriteStartElement(table.ExtendedProperties("DataSetName").ToString)
        xw.WriteWhitespace(vbLf)
        For i = 0 To view.Count - 1
            child1 = False
            child2 = False
            If Not table.Rows(i).RowState = DataRowState.Deleted Then
                xw.WriteString(vbTab)
                xw.WriteStartElement(table.TableName)
                xw.WriteString(vbLf)
                For x = 0 To table.Rows(i).ItemArray.Length - 1
                    Dim c As DataColumn = table.Columns(x)
                    If c.ColumnName = "REORDERMINIMUM" Then
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteStartElement("REORDERDAYSDELAY")
                        xw.WriteWhitespace(vbLf)
                        child1 = True
                        child2 = False
                    ElseIf c.ColumnName = "INITIAL" Then
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteEndElement()
                        xw.WriteWhitespace(vbLf)
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteStartElement("CASH")
                        xw.WriteWhitespace(vbLf)
                        child1 = True
                        child2 = False
                    ElseIf c.ColumnName = "INCREMENT" Then
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteStartElement("DAILY")
                        xw.WriteWhitespace(vbLf)
                        child1 = False
                        child2 = True
                    ElseIf c.ColumnName = "COOLMINIMUM" Then
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteEndElement()
                        xw.WriteWhitespace(vbLf)
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteEndElement()
                        xw.WriteWhitespace(vbLf)
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteStartElement("COOLNESS")
                        xw.WriteWhitespace(vbLf)
                        child1 = True
                        child2 = False
                    ElseIf c.ColumnName = "ARMS_DEALER_HANDGUNCLASS" Then
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteEndElement()
                        xw.WriteWhitespace(vbLf)
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        xw.WriteStartElement("BASICDEALERFLAGS")
                        xw.WriteWhitespace(vbLf)
                        child1 = True
                        child2 = False
                    End If
                    If Not trim OrElse (c Is table.PrimaryKey(0) OrElse ((c.DataType.Equals(GetType(Boolean)) OrElse c.DataType.Equals(GetType(Decimal)) OrElse c.DataType.Equals(GetType(ULong)) OrElse c.DataType.Equals(GetType(Integer)) OrElse c.DataType.Equals(GetType(Long))) AndAlso view(i)(c.ColumnName) <> 0) _
                        OrElse (c.DataType.Equals(GetType(String)) AndAlso view(i)(c.ColumnName) <> "")) Then
                        xw.WriteString(vbTab)
                        xw.WriteString(vbTab)
                        If child1 = True Then xw.WriteString(vbTab)
                        If child2 = True Then
                            xw.WriteString(vbTab)
                            xw.WriteString(vbTab)
                        End If
                        If Not c.DataType.Equals(GetType(Boolean)) Then
                            xw.WriteElementString(c.ColumnName, view(i)(c.ColumnName))
                        Else
                            If view(i)(c.ColumnName) Then
                                xw.WriteElementString(c.ColumnName, 1)
                            Else
                                xw.WriteElementString(c.ColumnName, 0)
                            End If
                        End If
                        xw.WriteString(vbLf)
                    End If
                Next

                xw.WriteString(vbTab)
                xw.WriteString(vbTab)
                xw.WriteEndElement()
                xw.WriteString(vbLf)

                xw.WriteString(vbTab)
                xw.WriteEndElement()
                xw.WriteString(vbLf)
            End If
        Next
        xw.WriteEndElement()
        xw.WriteEndDocument()
        xw.Close()

    End Sub

End Class
