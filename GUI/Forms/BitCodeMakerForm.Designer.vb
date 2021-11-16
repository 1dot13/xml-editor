<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class BitCodeMakerForm
    Inherits System.Windows.Forms.Form

    'Form overrides dispose to clean up the component list.
    <System.Diagnostics.DebuggerNonUserCode()> _
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Required by the Windows Form Designer
    Private components As System.ComponentModel.IContainer

    'NOTE: The following procedure is required by the Windows Form Designer
    'It can be modified using the Windows Form Designer.  
    'Do not modify it using the code editor.
    <System.Diagnostics.DebuggerStepThrough()> _
    Private Sub InitializeComponent()
        Me.CloseButton = New System.Windows.Forms.Button()
        Me.Label1 = New System.Windows.Forms.Label()
        Me.CodeTextBox = New System.Windows.Forms.TextBox()
        Me.AddButton = New System.Windows.Forms.Button()
        Me.NameTextBox = New System.Windows.Forms.TextBox()
        Me.Label2 = New System.Windows.Forms.Label()
        Me.CheckBoxPanel = New System.Windows.Forms.TableLayoutPanel()
        Me.Panel1 = New System.Windows.Forms.Panel()
        Me.Panel1.SuspendLayout()
        Me.SuspendLayout()
        '
        'CloseButton
        '
        Me.CloseButton.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.CloseButton.DialogResult = System.Windows.Forms.DialogResult.Cancel
        Me.CloseButton.Location = New System.Drawing.Point(547, 281)
        Me.CloseButton.Name = "CloseButton"
        Me.CloseButton.Size = New System.Drawing.Size(90, 24)
        Me.CloseButton.TabIndex = 2
        Me.CloseButton.Text = "Close"
        Me.CloseButton.UseVisualStyleBackColor = True
        '
        'Label1
        '
        Me.Label1.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.Label1.AutoSize = True
        Me.Label1.Location = New System.Drawing.Point(5, 287)
        Me.Label1.Name = "Label1"
        Me.Label1.Size = New System.Drawing.Size(35, 13)
        Me.Label1.TabIndex = 4
        Me.Label1.Text = "Code:"
        '
        'CodeTextBox
        '
        Me.CodeTextBox.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.CodeTextBox.Location = New System.Drawing.Point(46, 284)
        Me.CodeTextBox.Name = "CodeTextBox"
        Me.CodeTextBox.Size = New System.Drawing.Size(139, 20)
        Me.CodeTextBox.TabIndex = 5
        '
        'AddButton
        '
        Me.AddButton.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.AddButton.Location = New System.Drawing.Point(435, 281)
        Me.AddButton.Name = "AddButton"
        Me.AddButton.Size = New System.Drawing.Size(90, 24)
        Me.AddButton.TabIndex = 6
        Me.AddButton.Text = "Add To Table"
        Me.AddButton.UseVisualStyleBackColor = True
        '
        'NameTextBox
        '
        Me.NameTextBox.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.NameTextBox.Location = New System.Drawing.Point(239, 284)
        Me.NameTextBox.Name = "NameTextBox"
        Me.NameTextBox.Size = New System.Drawing.Size(190, 20)
        Me.NameTextBox.TabIndex = 8
        '
        'Label2
        '
        Me.Label2.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.Label2.AutoSize = True
        Me.Label2.Location = New System.Drawing.Point(198, 287)
        Me.Label2.Name = "Label2"
        Me.Label2.Size = New System.Drawing.Size(38, 13)
        Me.Label2.TabIndex = 7
        Me.Label2.Text = "Name:"
        '
        'CheckBoxPanel
        '
        Me.CheckBoxPanel.ColumnCount = 20
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.CheckBoxPanel.Dock = System.Windows.Forms.DockStyle.Fill
        Me.CheckBoxPanel.Location = New System.Drawing.Point(0, 0)
        Me.CheckBoxPanel.Name = "CheckBoxPanel"
        Me.CheckBoxPanel.RowCount = 40
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.RowStyles.Add(New System.Windows.Forms.RowStyle())
        Me.CheckBoxPanel.Size = New System.Drawing.Size(635, 276)
        Me.CheckBoxPanel.TabIndex = 9
        '
        'Panel1
        '
        Me.Panel1.Anchor = CType((((System.Windows.Forms.AnchorStyles.Top Or System.Windows.Forms.AnchorStyles.Bottom) _
            Or System.Windows.Forms.AnchorStyles.Left) _
            Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.Panel1.Controls.Add(Me.CheckBoxPanel)
        Me.Panel1.Location = New System.Drawing.Point(2, 2)
        Me.Panel1.Name = "Panel1"
        Me.Panel1.Size = New System.Drawing.Size(635, 276)
        Me.Panel1.TabIndex = 10
        '
        'BitCodeMakerForm
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.ClientSize = New System.Drawing.Size(640, 307)
        Me.Controls.Add(Me.Panel1)
        Me.Controls.Add(Me.NameTextBox)
        Me.Controls.Add(Me.Label2)
        Me.Controls.Add(Me.AddButton)
        Me.Controls.Add(Me.CodeTextBox)
        Me.Controls.Add(Me.Label1)
        Me.Controls.Add(Me.CloseButton)
        Me.MinimumSize = New System.Drawing.Size(656, 345)
        Me.Name = "BitCodeMakerForm"
        Me.ShowIcon = False
        Me.ShowInTaskbar = False
        Me.Text = "Bitwise Code Maker"
        Me.Panel1.ResumeLayout(False)
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub
    Friend WithEvents CloseButton As System.Windows.Forms.Button
    Friend WithEvents Label1 As System.Windows.Forms.Label
    Friend WithEvents CodeTextBox As System.Windows.Forms.TextBox
    Friend WithEvents AddButton As System.Windows.Forms.Button
    Friend WithEvents NameTextBox As System.Windows.Forms.TextBox
    Friend WithEvents Label2 As System.Windows.Forms.Label
    Friend WithEvents CheckBoxPanel As System.Windows.Forms.TableLayoutPanel
    Friend WithEvents Panel1 As System.Windows.Forms.Panel
End Class
