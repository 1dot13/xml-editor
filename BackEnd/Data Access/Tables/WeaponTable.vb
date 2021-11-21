Imports System.Xml
Imports System.IO
Imports System.Xml.Linq
Imports System.Data

Public Class WeaponTable
    Inherits DefaultTable

    Public Sub New(ByVal table As DataTable, ByVal manager As DataManager)
        MyBase.New(table, manager)
    End Sub

    Public Overrides Sub SaveData()
        Dim memStream As New MemoryStream
        WriteXml(_table, memStream)
        memStream.Seek(0, SeekOrigin.Begin)

        Dim doc As XElement = XElement.Load(memStream)
        Dim element As XElement
        For Each element In doc.Descendants()
            If element.Name.LocalName.StartsWith("BarrelConf") Then
                Dim nodeName As XName = "BarrelConfiguration"
                element.Name = nodeName
            End If
        Next

        Dim fileName As String = _table.GetStringProperty(TableProperty.FileName)
        Dim filePath As String = GetFilePath(fileName)
        doc.Save(filePath)

        _table.AcceptChanges()
    End Sub

    Public Overrides Sub LoadData()
        _table.BeginLoadData()
        _table.Clear()

        Dim fileName As String = _table.GetStringProperty(TableProperty.FileName)
        Dim filePath As String = GetFilePath(fileName)
        Dim doc As XElement = XElement.Load(filePath)

        ' Make <BarrelConfiguration> tags unique
        Dim i As Integer = 0
        Dim iMax As Integer = 0
        Dim element As XElement
        For Each element In doc.Descendants()
            If element.Name.LocalName = "WEAPON" Then i = 0

            If element.Name.LocalName.StartsWith("BarrelConf") Then
                Dim nodeName As XName = $"BarrelConfiguration{i}"
                element.Name = nodeName
                i += 1

                If i > iMax Then iMax = i
            End If
        Next

        ' Add unique barrelconfiguration columns to WeaponTable so we can load the data using default ReadXml function
        For i = 0 To (iMax - 1)
            _table.Columns.Add(MakeColumn($"BarrelConfiguration{i}", $"BarrelConfiguration{i}", GetType(Integer), , , , , , , True))
        Next

        Dim xmlFile As XmlReader = doc.CreateReader()
        _table.ReadXml(xmlFile)
        _table.EndLoadData()
    End Sub
End Class
