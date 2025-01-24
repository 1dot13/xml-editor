Imports System.Data

Public Class ItemDataForm
    Inherits BaseDataForm

    Public Sub New(manager As DataManager, ByVal itemID As Integer, ByVal formText As String)
        MyBase.New(manager, itemID, formText)
        ' This call is required by the Windows Form Designer.
        InitializeComponent()

        ' Add any initialization after the InitializeComponent() call.
        Initialize()
    End Sub

    Private Sub RemoveUnusedLanguageSpecificItemTabs()
        Dim baseItemFolder As String = "Items\"
        Dim baseItemName As String = "Items.xml"

        If LanguageSpecificFileExist(baseItemFolder & OtherText.German & "." & baseItemName) = False Then
            Me.TabControl2.Controls.Remove(Me.GermanPage)
        End If
        If LanguageSpecificFileExist(baseItemFolder & OtherText.Russian & "." & baseItemName) = False Then
            Me.TabControl2.Controls.Remove(Me.RussianPage)
        End If
        If LanguageSpecificFileExist(baseItemFolder & OtherText.Polish & "." & baseItemName) = False Then
            Me.TabControl2.Controls.Remove(Me.PolishPage)
        End If
        If LanguageSpecificFileExist(baseItemFolder & OtherText.Italian & "." & baseItemName) = False Then
            Me.TabControl2.Controls.Remove(Me.ItalianPage)
        End If
        If LanguageSpecificFileExist(baseItemFolder & OtherText.French & "." & baseItemName) = False Then
            Me.TabControl2.Controls.Remove(Me.FrenchPage)
        End If
        If LanguageSpecificFileExist(baseItemFolder & OtherText.Chinese & "." & baseItemName) = False Then
            Me.TabControl2.Controls.Remove(Me.ChinesePage)
        End If
        If LanguageSpecificFileExist(baseItemFolder & OtherText.Dutch & "." & baseItemName) = False Then
            Me.TabControl2.Controls.Remove(Me.DutchPage)
        End If

    End Sub

    Private Function LanguageSpecificFileExist(ByVal filename As String) As Boolean
        Dim exists As Boolean = True

        Dim filePath = _dm.GetLanguageSpecificTableDirectory(filename) & filename

        If System.IO.File.Exists(filePath) = False Then
            exists = False
        End If

        Return exists

    End Function

    Public itemFlagsArray()
    Public itemFlags2Array()

    Protected Sub Initialize()

        ItemSizeUpDown.Maximum = ItemSizeMax

        ' RoWa21: Only add the language specific Tabs for XML-files that exist
        RemoveUnusedLanguageSpecificItemTabs()

        Bind(Tables.Items.Name, Tables.Items.Fields.ID & "=" & _id)

        ClassNameLabel.DataBindings.Add("Text", usItemClassComboBox, "Text")

        PopulateGraphicsTypeComboBox()

        GraphicTypeComboBox.SelectedIndex = _view(0)(Tables.Items.Fields.GraphicType)
        GraphicIndexUpDown.Maximum = _dm.ItemImages.SmallItems(GraphicTypeComboBox.SelectedIndex).Length - 1
        GraphicIndexUpDown.Value = _view(0)(Tables.Items.Fields.GraphicIndex)

        DisplayTabs()

        'Setting Item Flags
        itemFlagsArray = {
            ItemFlags.Bloodbag, 'Dummy value to keep the array indexes correct
            ItemFlags.Manpad,
            ItemFlags.Beartrap,
            ItemFlags.Camera,
            ItemFlags.Waterdrum,
            ItemFlags.BloodcatMeat,
            ItemFlags.CowMeat,
            ItemFlags.Beltfed,
            ItemFlags.Ammobelt,
            ItemFlags.AmmobeltVest,
            ItemFlags.CamoRemoval,
            ItemFlags.Cleaningkit,
            ItemFlags.AttentionItem,
            ItemFlags.Garotte,
            ItemFlags.Covert,
            ItemFlags.Corpse,
            ItemFlags.BloodcatSkin,
            ItemFlags.NoMetalDetection,
            ItemFlags.JumpGrenade,
            ItemFlags.Handcuffs,
            ItemFlags.Taser,
            ItemFlags.ScubaBottle,
            ItemFlags.ScubaMask,
            ItemFlags.ScubaFins,
            ItemFlags.TripwireRoll,
            ItemFlags.Radioset,
            ItemFlags.SignalShell,
            ItemFlags.Soda,
            ItemFlags.RoofcollapseItem,
            ItemFlags.DiseaseprotectionFace,
            ItemFlags.DiseaseprotectionHand,
            ItemFlags.LBEexplosionproof,
            ItemFlags.EmptyBloodbag,
            ItemFlags.MedicalSplint,
            DamageableCheckBox,
            RepairableCheckBox,
            WaterDamagesCheckBox,
            MetalCheckBox,
            SinksCheckBox,
            ShowStatusCheckBox,
            HiddenAddonCheckBox,
            TwoHandedCheckBox,
            NotBuyableCheckBox,
            AttachmentCheckBox,
            HiddenAttachmentCheckBox,
            BigGunListCheckBox,
            NotInEditorCheckBox,
            DefaultUndroppableCheckBox,
            UnaerodynamicCheckBox,
            ElectronicCheckBox,
            CannonRadioButton,
            RocketRifleCheckBox,
            FingerPrintIDCheckBox,
            MetalDetectorCheckBox,
            GasMaskCheckBox,
            LockBombCheckBox,
            FlareCheckBox,
            GrenadeLauncherRadioButton,
            MortarRadioButton,
            DuckbillCheckBox,
            ItemFlags.Detonator,
            ItemFlags.RemoteDetonator,
            HideMuzzleFlashCheckBox,
            RocketLauncherRadioButton
        }

        itemFlags2Array = {
            SingleShotRocketLauncherCheckBox,
            BrassKnucklesCheckBox,
            CrowbarCheckBox,
            GLGrenadeCheckBox,
            FlakJacketCheckBox,
            LeatherJacketCheckBox,
            BatteriesCheckBox,
            NeedsBatteriesCheckBox,
            XRayCheckBox,
            WireCuttersCheckBox,
            ToolKitCheckBox,
            FirstAidKitCheckBox,
            MedicalKitCheckBox,
            CanteenCheckBox,
            JarCheckBox,
            CanAndStringCheckBox,
            MarblesCheckBox,
            WalkmanCheckBox,
            RemoteTriggerCheckBox,
            RobotRemoteControlCheckBox,
            CamoKitCheckBox,
            LocksmithKitCheckBox,
            MineCheckBox,
            ItemFlags2.AntitankMine,
            HardwareCheckBox,
            MedicalCheckBox,
            GasCanCheckBox,
            ContainsLiquidCheckBox,
            RockCheckBox,
            ThermalOpticsCheckBox,
            SciFiCheckBox,
            NewInventoryCheckBox,
            ItemFlags2.DiseaseSystemExclusive,
            BarrelCheckBox,
            TripWireActivationCheckBox,
            TripWireCheckBox,
            DirectionalCheckBox,
            BlockIronSightCheckBox,
            AllowClimbingCheckbox,
            ItemFlags2.Cigarette,
            ItemFlags2.ProvidesRobotCamo,
            ItemFlags2.ProvidesRobotNightVision,
            ItemFlags2.ProvidesRobotLaserBonus
        }

        'ItemFlag1
        Dim TempChecklistBox As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        TempChecklistBox.SetItemChecked(ItemFlags.Bloodbag, _view(0)("Bloodbag"))
        TempChecklistBox.SetItemChecked(ItemFlags.Manpad, _view(0)("Manpad"))
        TempChecklistBox.SetItemChecked(ItemFlags.Beartrap, _view(0)("Beartrap"))
        TempChecklistBox.SetItemChecked(ItemFlags.Camera, _view(0)("Camera"))
        TempChecklistBox.SetItemChecked(ItemFlags.Waterdrum, _view(0)("Waterdrum"))
        TempChecklistBox.SetItemChecked(ItemFlags.BloodcatMeat, _view(0)("BloodcatMeat"))
        TempChecklistBox.SetItemChecked(ItemFlags.CowMeat, _view(0)("CowMeat"))
        TempChecklistBox.SetItemChecked(ItemFlags.Beltfed, _view(0)("Beltfed"))
        TempChecklistBox.SetItemChecked(ItemFlags.Ammobelt, _view(0)("Ammobelt"))
        TempChecklistBox.SetItemChecked(ItemFlags.AmmobeltVest, _view(0)("AmmobeltVest"))
        TempChecklistBox.SetItemChecked(ItemFlags.CamoRemoval, _view(0)("CamoRemoval"))
        TempChecklistBox.SetItemChecked(ItemFlags.Cleaningkit, _view(0)("Cleaningkit"))
        TempChecklistBox.SetItemChecked(ItemFlags.AttentionItem, _view(0)("AttentionItem"))
        TempChecklistBox.SetItemChecked(ItemFlags.Garotte, _view(0)("Garotte"))
        TempChecklistBox.SetItemChecked(ItemFlags.Covert, _view(0)("Covert"))
        TempChecklistBox.SetItemChecked(ItemFlags.Corpse, _view(0)("Corpse"))
        TempChecklistBox.SetItemChecked(ItemFlags.BloodcatSkin, _view(0)("BloodcatSkin"))
        TempChecklistBox.SetItemChecked(ItemFlags.NoMetalDetection, _view(0)("NoMetalDetection"))
        TempChecklistBox.SetItemChecked(ItemFlags.JumpGrenade, _view(0)("JumpGrenade"))
        TempChecklistBox.SetItemChecked(ItemFlags.Handcuffs, _view(0)("Handcuffs"))
        TempChecklistBox.SetItemChecked(ItemFlags.Taser, _view(0)("Taser"))
        TempChecklistBox.SetItemChecked(ItemFlags.ScubaBottle, _view(0)("ScubaBottle"))
        TempChecklistBox.SetItemChecked(ItemFlags.ScubaMask, _view(0)("ScubaMask"))
        TempChecklistBox.SetItemChecked(ItemFlags.ScubaFins, _view(0)("ScubaFins"))
        TempChecklistBox.SetItemChecked(ItemFlags.TripwireRoll, _view(0)("TripwireRoll"))
        TempChecklistBox.SetItemChecked(ItemFlags.Radioset, _view(0)("Radioset"))
        TempChecklistBox.SetItemChecked(ItemFlags.SignalShell, _view(0)("SignalShell"))
        TempChecklistBox.SetItemChecked(ItemFlags.Soda, _view(0)("Soda"))
        TempChecklistBox.SetItemChecked(ItemFlags.RoofcollapseItem, _view(0)("RoofcollapseItem"))
        TempChecklistBox.SetItemChecked(ItemFlags.DiseaseprotectionFace, _view(0)("DiseaseprotectionFace"))
        TempChecklistBox.SetItemChecked(ItemFlags.DiseaseprotectionHand, _view(0)("DiseaseprotectionHand"))
        TempChecklistBox.SetItemChecked(ItemFlags.LBEexplosionproof, _view(0)("LBEexplosionproof"))
        TempChecklistBox.SetItemChecked(ItemFlags.EmptyBloodbag, _view(0)("EmptyBloodbag"))
        TempChecklistBox.SetItemChecked(ItemFlags.MedicalSplint, _view(0)("MedicalSplint"))
        TempChecklistBox.SetItemChecked(ItemFlags.Damageable, _view(0)("Damageable"))
        TempChecklistBox.SetItemChecked(ItemFlags.Repairable, _view(0)("Repairable"))
        TempChecklistBox.SetItemChecked(ItemFlags.WaterDamages, _view(0)("WaterDamages"))
        TempChecklistBox.SetItemChecked(ItemFlags.Metal, _view(0)("Metal"))
        TempChecklistBox.SetItemChecked(ItemFlags.Sinks, _view(0)("Sinks"))
        TempChecklistBox.SetItemChecked(ItemFlags.ShowStatus, _view(0)("ShowStatus"))
        TempChecklistBox.SetItemChecked(ItemFlags.HiddenAddon, _view(0)("HiddenAddon"))
        TempChecklistBox.SetItemChecked(ItemFlags.TwoHanded, _view(0)("TwoHanded"))
        TempChecklistBox.SetItemChecked(ItemFlags.NotBuyable, _view(0)("NotBuyable"))
        TempChecklistBox.SetItemChecked(ItemFlags.Attachment, _view(0)("Attachment"))
        TempChecklistBox.SetItemChecked(ItemFlags.HiddenAttachment, _view(0)("HiddenAttachment"))
        TempChecklistBox.SetItemChecked(ItemFlags.BigGunList, _view(0)("BigGunList"))
        TempChecklistBox.SetItemChecked(ItemFlags.NotInEditor, _view(0)("NotInEditor"))
        TempChecklistBox.SetItemChecked(ItemFlags.DefaultUndroppable, _view(0)("DefaultUndroppable"))
        TempChecklistBox.SetItemChecked(ItemFlags.Unaerodynamic, _view(0)("Unaerodynamic"))
        TempChecklistBox.SetItemChecked(ItemFlags.Electronic, _view(0)("Electronic"))
        TempChecklistBox.SetItemChecked(ItemFlags.Cannon, _view(0)("Cannon"))
        TempChecklistBox.SetItemChecked(ItemFlags.RocketRifle, _view(0)("RocketRifle"))
        TempChecklistBox.SetItemChecked(ItemFlags.FingerPrintID, _view(0)("FingerPrintID"))
        TempChecklistBox.SetItemChecked(ItemFlags.MetalDetector, _view(0)("MetalDetector"))
        TempChecklistBox.SetItemChecked(ItemFlags.GasMask, _view(0)("GasMask"))
        TempChecklistBox.SetItemChecked(ItemFlags.LockBomb, _view(0)("LockBomb"))
        TempChecklistBox.SetItemChecked(ItemFlags.Flare, _view(0)("Flare"))
        TempChecklistBox.SetItemChecked(ItemFlags.GrenadeLauncher, _view(0)("GrenadeLauncher"))
        TempChecklistBox.SetItemChecked(ItemFlags.Mortar, _view(0)("Mortar"))
        TempChecklistBox.SetItemChecked(ItemFlags.Duckbill, _view(0)("Duckbill"))
        TempChecklistBox.SetItemChecked(ItemFlags.Detonator, _view(0)("Detonator"))
        TempChecklistBox.SetItemChecked(ItemFlags.RemoteDetonator, _view(0)("RemoteDetonator"))
        TempChecklistBox.SetItemChecked(ItemFlags.HideMuzzleFlash, _view(0)("HideMuzzleFlash"))
        TempChecklistBox.SetItemChecked(ItemFlags.RocketLauncher, _view(0)("RocketLauncher"))

        'ItemFlag2
        TempChecklistBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        TempChecklistBox.SetItemChecked(ItemFlags2.SingleShotRocketLauncher, _view(0)("SingleShotRocketLauncher"))
        TempChecklistBox.SetItemChecked(ItemFlags2.BrassKnuckles, _view(0)("BrassKnuckles"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Crowbar, _view(0)("Crowbar"))
        TempChecklistBox.SetItemChecked(ItemFlags2.GLGrenade, _view(0)("GLGrenade"))
        TempChecklistBox.SetItemChecked(ItemFlags2.FlakJacket, _view(0)("FlakJacket"))
        TempChecklistBox.SetItemChecked(ItemFlags2.LeatherJacket, _view(0)("LeatherJacket"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Batteries, _view(0)("Batteries"))
        TempChecklistBox.SetItemChecked(ItemFlags2.NeedsBatteries, _view(0)("NeedsBatteries"))
        TempChecklistBox.SetItemChecked(ItemFlags2.XRay, _view(0)("XRay"))
        TempChecklistBox.SetItemChecked(ItemFlags2.WireCutters, _view(0)("WireCutters"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Toolkit, _view(0)("Toolkit"))
        TempChecklistBox.SetItemChecked(ItemFlags2.FirstAidKit, _view(0)("FirstAidKit"))
        TempChecklistBox.SetItemChecked(ItemFlags2.MedicalKit, _view(0)("MedicalKit"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Canteen, _view(0)("Canteen"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Jar, _view(0)("Jar"))
        TempChecklistBox.SetItemChecked(ItemFlags2.CanAndString, _view(0)("CanAndString"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Marbles, _view(0)("Marbles"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Walkman, _view(0)("Walkman"))
        TempChecklistBox.SetItemChecked(ItemFlags2.RemoteTrigger, _view(0)("RemoteTrigger"))
        TempChecklistBox.SetItemChecked(ItemFlags2.RobotRemoteControl, _view(0)("RobotRemoteControl"))
        TempChecklistBox.SetItemChecked(ItemFlags2.CamouflageKit, _view(0)("CamouflageKit"))
        TempChecklistBox.SetItemChecked(ItemFlags2.LocksmithKit, _view(0)("LocksmithKit"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Mine, _view(0)("Mine"))
        TempChecklistBox.SetItemChecked(ItemFlags2.AntitankMine, _view(0)("AntitankMine"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Hardware, _view(0)("Hardware"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Medical, _view(0)("Medical"))
        TempChecklistBox.SetItemChecked(ItemFlags2.GasCan, _view(0)("GasCan"))
        TempChecklistBox.SetItemChecked(ItemFlags2.ContainsLiquid, _view(0)("ContainsLiquid"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Rock, _view(0)("Rock"))
        TempChecklistBox.SetItemChecked(ItemFlags2.ThermalOptics, _view(0)("ThermalOptics"))
        TempChecklistBox.SetItemChecked(ItemFlags2.SciFi, _view(0)("SciFi"))
        TempChecklistBox.SetItemChecked(ItemFlags2.NewInv, _view(0)("NewInv"))
        TempChecklistBox.SetItemChecked(ItemFlags2.DiseaseSystemExclusive, _view(0)("DiseaseSystemExclusive"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Barrel, _view(0)("Barrel"))
        TempChecklistBox.SetItemChecked(ItemFlags2.TripWireActivation, _view(0)("TripWireActivation"))
        TempChecklistBox.SetItemChecked(ItemFlags2.TripWire, _view(0)("TripWire"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Directional, _view(0)("Directional"))
        TempChecklistBox.SetItemChecked(ItemFlags2.BlockIronSight, _view(0)("BlockIronSight"))
        TempChecklistBox.SetItemChecked(ItemFlags2.AllowClimbing, _view(0)("AllowClimbing"))
        TempChecklistBox.SetItemChecked(ItemFlags2.Cigarette, _view(0)("Cigarette"))
        TempChecklistBox.SetItemChecked(ItemFlags2.ProvidesRobotCamo, _view(0)("ProvidesRobotCamo"))
        TempChecklistBox.SetItemChecked(ItemFlags2.ProvidesRobotNightVision, _view(0)("ProvidesRobotNightVision"))
        TempChecklistBox.SetItemChecked(ItemFlags2.ProvidesRobotLaserBonus, _view(0)("ProvidesRobotLaserBonus"))



        ''Setting Drugs
        'Dim TempDrugFlags As UInt32 = _view(0)("DrugType")
        'Dim BitDrugFlags As String = ToBinary(TempDrugFlags).PadRight(32, "0")

        ''TODO: Dynamically create the checkboxes from the entries in table "DRUG"

        'Dim TempDrugChecklistBox As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox65").Controls.Item("DrugsCheckedList")
        'For i As Integer = 0 To 31
        '    TempDrugChecklistBox.SetItemChecked(i, (BitDrugFlags(i).ToString.Equals("1")))
        'Next


        SetupGrid(AttachmentGrid, Tables.Attachments.Name, Tables.Attachments.Fields.ItemID)
        SetupGrid(AttachmentInfoGrid, Tables.AttachmentInfo.Name, Tables.AttachmentInfo.Fields.ItemID)
        SetupGrid(AttachToGrid, Tables.Attachments.Name, Tables.Attachments.Fields.AttachmentID)
        SetupGrid(IncompatibleAttachmentGrid, Tables.IncompatibleAttachments.Name, Tables.IncompatibleAttachments.Fields.ItemID)
        SetupGrid(LaunchableGrid, Tables.Launchables.Name, Tables.Launchables.Fields.ItemID)
        SetupGrid(LauncherGrid, Tables.Launchables.Name, Tables.Launchables.Fields.LaunchableID)
        SetupGrid(CompatibleFaceItemGrid, Tables.CompatibleFaceItems.Name, Tables.CompatibleFaceItems.Fields.ItemID)

        LoadInventoryData()
    End Sub

    Protected Sub PopulateGraphicsTypeComboBox()
        Me.GraphicTypeComboBox.Items.Clear()
        For i As Integer = 0 To _dm.ImageTypeCount
            If i = 0 Then
                Me.GraphicTypeComboBox.Items.Add("Guns")
            Else
                Me.GraphicTypeComboBox.Items.Add(String.Format("P{0}Items", i))
            End If
        Next
    End Sub

    'Create links between Flags tab checkboxes and same flags distributed to other tabs
    Private Sub Damageable_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DamageableCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Damageable, DamageableCheckBox.Checked)
    End Sub

    Private Sub Repairable_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RepairableCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Repairable, RepairableCheckBox.Checked)
    End Sub

    Private Sub WaterDamages_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles WaterDamagesCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.WaterDamages, WaterDamagesCheckBox.Checked)
    End Sub

    Private Sub Metal_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MetalCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Metal, MetalCheckBox.Checked)
    End Sub

    Private Sub Sinks_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles SinksCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Sinks, SinksCheckBox.Checked)
    End Sub

    Private Sub ShowStatus_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ShowStatusCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.ShowStatus, ShowStatusCheckBox.Checked)
    End Sub

    Private Sub HiddenAddon_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles HiddenAddonCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.HiddenAddon, HiddenAddonCheckBox.Checked)
    End Sub

    Private Sub TwoHanded_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles TwoHandedCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.TwoHanded, TwoHandedCheckBox.Checked)
    End Sub

    Private Sub NotBuyable_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles NotBuyableCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.NotBuyable, NotBuyableCheckBox.Checked)
    End Sub

    Private Sub Attachment_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles AttachmentCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Attachment, AttachmentCheckBox.Checked)
    End Sub

    Private Sub HiddenAttachment_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles HiddenAttachmentCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.HiddenAttachment, HiddenAttachmentCheckBox.Checked)
    End Sub

    Private Sub BigGunList_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles BigGunListCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.BigGunList, BigGunListCheckBox.Checked)
    End Sub

    Private Sub NotInEditor_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles NotInEditorCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.NotInEditor, NotInEditorCheckBox.Checked)
    End Sub

    Private Sub DefaultUndroppable_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DefaultUndroppableCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.DefaultUndroppable, DefaultUndroppableCheckBox.Checked)
    End Sub

    Private Sub Unaerodynamic_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles UnaerodynamicCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Unaerodynamic, UnaerodynamicCheckBox.Checked)
    End Sub

    Private Sub Electronic_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ElectronicCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Electronic, ElectronicCheckBox.Checked)
    End Sub

    Private Sub Cannon_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles CannonRadioButton.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Cannon, CannonRadioButton.Checked)
    End Sub

    Private Sub RocketRifle_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RocketRifleCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.RocketRifle, RocketRifleCheckBox.Checked)
    End Sub

    Private Sub FingerPrintID_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles FingerPrintIDCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.FingerPrintID, FingerPrintIDCheckBox.Checked)
    End Sub

    Private Sub MetalDetector_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MetalDetectorCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.MetalDetector, MetalDetectorCheckBox.Checked)
    End Sub

    Private Sub GasMask_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GasMaskCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.GasMask, GasMaskCheckBox.Checked)
    End Sub

    Private Sub LockBomb_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles LockBombCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.LockBomb, LockBombCheckBox.Checked)
    End Sub

    Private Sub Flare_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles FlareCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Flare, FlareCheckBox.Checked)
    End Sub

    Private Sub GrenadeLauncher_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GrenadeLauncherRadioButton.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.GrenadeLauncher, GrenadeLauncherRadioButton.Checked)
    End Sub

    Private Sub Mortar_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MortarRadioButton.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Mortar, MortarRadioButton.Checked)
    End Sub

    Private Sub Duckbill_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DuckbillCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.Duckbill, DuckbillCheckBox.Checked)
    End Sub

    Private Sub HideMuzzleFlash_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles HideMuzzleFlashCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.HideMuzzleFlash, HideMuzzleFlashCheckBox.Checked)
    End Sub

    Private Sub RocketLauncher_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RocketLauncherRadioButton.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
        checklist.SetItemChecked(ItemFlags.RocketLauncher, RocketLauncherRadioButton.Checked)
    End Sub

    Private Sub ItemFlag_ValueChanged(ByVal sender As System.Object, ByVal e As System.Windows.Forms.ItemCheckEventArgs) Handles ItemFlagsCheckedList.ItemCheck
        Select Case e.Index
            ' No checkboxes yet
            Case ItemFlags.Bloodbag
            Case ItemFlags.Manpad
            Case ItemFlags.Beartrap
            Case ItemFlags.Camera
            Case ItemFlags.Waterdrum
            Case ItemFlags.BloodcatMeat
            Case ItemFlags.CowMeat
            Case ItemFlags.Beltfed
            Case ItemFlags.Ammobelt
            Case ItemFlags.AmmobeltVest
            Case ItemFlags.CamoRemoval
            Case ItemFlags.Cleaningkit
            Case ItemFlags.AttentionItem
            Case ItemFlags.Garotte
            Case ItemFlags.Covert
            Case ItemFlags.Corpse
            Case ItemFlags.BloodcatSkin
            Case ItemFlags.NoMetalDetection
            Case ItemFlags.JumpGrenade
            Case ItemFlags.Handcuffs
            Case ItemFlags.Taser
            Case ItemFlags.ScubaBottle
            Case ItemFlags.ScubaMask
            Case ItemFlags.ScubaFins
            Case ItemFlags.TripwireRoll
            Case ItemFlags.Radioset
            Case ItemFlags.SignalShell
            Case ItemFlags.Soda
            Case ItemFlags.RoofcollapseItem
            Case ItemFlags.DiseaseprotectionFace
            Case ItemFlags.DiseaseprotectionHand
            Case ItemFlags.LBEexplosionproof
            Case ItemFlags.EmptyBloodbag
            Case ItemFlags.MedicalSplint
            Case ItemFlags.Detonator
            Case ItemFlags.RemoteDetonator

            Case Else
                itemFlagsArray(e.Index).Checked = e.NewValue
        End Select
    End Sub

    Private Sub SingleShotRocketLauncher_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles SingleShotRocketLauncherCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.SingleShotRocketLauncher, SingleShotRocketLauncherCheckBox.Checked)
    End Sub

    Private Sub BrassKnuckles_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles BrassKnucklesCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.BrassKnuckles, BrassKnucklesCheckBox.Checked)
    End Sub

    Private Sub Crowbar_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles CrowbarCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Crowbar, CrowbarCheckBox.Checked)
    End Sub

    Private Sub GLGrenade_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GLGrenadeCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.GLGrenade, GLGrenadeCheckBox.Checked)
    End Sub

    Private Sub Flakjacket_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles FlakJacketCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.FlakJacket, FlakJacketCheckBox.Checked)
    End Sub

    Private Sub LeatherJacket_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles LeatherJacketCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.LeatherJacket, LeatherJacketCheckBox.Checked)
    End Sub

    Private Sub Batteries_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles BatteriesCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Batteries, BatteriesCheckBox.Checked)
    End Sub

    Private Sub NeedsBatteries_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles NeedsBatteriesCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.NeedsBatteries, NeedsBatteriesCheckBox.Checked)
    End Sub

    Private Sub XRay_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles XRayCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.XRay, XRayCheckBox.Checked)
    End Sub

    Private Sub Wirecutters_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles WireCuttersCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.WireCutters, WireCuttersCheckBox.Checked)
    End Sub

    Private Sub Toolkit_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ToolKitCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Toolkit, ToolKitCheckBox.Checked)
    End Sub

    Private Sub FirstAidKit_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles FirstAidKitCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.FirstAidKit, FirstAidKitCheckBox.Checked)
    End Sub

    Private Sub MedicalKit_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MedicalKitCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.MedicalKit, MedicalKitCheckBox.Checked)
    End Sub

    Private Sub Canteen_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles CanteenCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Canteen, CanteenCheckBox.Checked)
    End Sub

    Private Sub Jar_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles JarCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Jar, JarCheckBox.Checked)
    End Sub

    Private Sub CanAndString_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles CanAndStringCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.CanAndString, CanAndStringCheckBox.Checked)
    End Sub

    Private Sub Marbles_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MarblesCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Marbles, MarblesCheckBox.Checked)
    End Sub

    Private Sub Walkman_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles WalkmanCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Walkman, WalkmanCheckBox.Checked)
    End Sub

    Private Sub RemoteTrigger_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RemoteTriggerCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.RemoteTrigger, RemoteTriggerCheckBox.Checked)
    End Sub

    Private Sub RobotRemoteControl_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RobotRemoteControlCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.RobotRemoteControl, RobotRemoteControlCheckBox.Checked)
    End Sub

    Private Sub CamoKit_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles CamoKitCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.CamouflageKit, CamoKitCheckBox.Checked)
    End Sub

    Private Sub LocksmithKit_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles LocksmithKitCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.LocksmithKit, LocksmithKitCheckBox.Checked)
    End Sub

    Private Sub Mine_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MineCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Mine, MineCheckBox.Checked)
    End Sub

    Private Sub Hardware_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles HardwareCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Hardware, HardwareCheckBox.Checked)
    End Sub

    Private Sub Medical_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MedicalCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Medical, MedicalCheckBox.Checked)
    End Sub

    Private Sub GasCan_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GasCanCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.GasCan, GasCanCheckBox.Checked)
    End Sub

    Private Sub ContainsLiquid_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ContainsLiquidCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.ContainsLiquid, ContainsLiquidCheckBox.Checked)
    End Sub

    Private Sub Rock_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RockCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Rock, RockCheckBox.Checked)
    End Sub

    Private Sub ThermalOptics_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ThermalOpticsCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.ThermalOptics, ThermalOpticsCheckBox.Checked)
    End Sub

    Private Sub Scifi_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles SciFiCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.SciFi, SciFiCheckBox.Checked)
    End Sub

    Private Sub NewInv_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles NewInventoryCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.NewInv, NewInventoryCheckBox.Checked)
    End Sub

    Private Sub Barrel_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles BarrelCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Barrel, BarrelCheckBox.Checked)
    End Sub

    Private Sub TripWireActivation_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles TripWireActivationCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.TripWireActivation, TripWireActivationCheckBox.Checked)
    End Sub

    Private Sub TripWire_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles TripWireCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.TripWire, TripWireCheckBox.Checked)
    End Sub

    Private Sub Directional_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DirectionalCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.Directional, DirectionalCheckBox.Checked)
    End Sub

    Private Sub BlockIronsight_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles BlockIronSightCheckBox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.BlockIronSight, BlockIronSightCheckBox.Checked)
    End Sub

    Private Sub AllowClimbing_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles AllowClimbingCheckbox.CheckedChanged
        Dim checklist As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
        checklist.SetItemChecked(ItemFlags2.AllowClimbing, AllowClimbingCheckbox.Checked)
    End Sub

    Private Sub ItemFlag2_ValueChanged(ByVal sender As System.Object, ByVal e As System.Windows.Forms.ItemCheckEventArgs) Handles ItemFlags2CheckedList.ItemCheck
        Select Case e.Index
            ' No checkboxes yet
            Case ItemFlags2.AntitankMine
            Case ItemFlags2.DiseaseSystemExclusive
            Case ItemFlags2.Cigarette
            Case ItemFlags2.ProvidesRobotCamo
            Case ItemFlags2.ProvidesRobotNightVision
            Case ItemFlags2.ProvidesRobotLaserBonus

            Case Else
                itemFlags2Array(e.Index).Checked = e.NewValue
        End Select
    End Sub

#Region " General Tab "
#End Region

#Region " Graphics Tab "
    Protected Sub LoadImages(ByVal type As Integer, ByVal index As Integer)
        SmallItemImage.Image = _dm.ItemImages.SmallItemImage(type, index)
        MediumItemImage.Image = _dm.ItemImages.MediumItemImage(type, index)
        BigItemImage.Image = _dm.ItemImages.BigItemImage(type, index)
    End Sub

    Private Sub GraphicIndexUpDown_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GraphicIndexUpDown.ValueChanged
        LoadImages(GraphicTypeComboBox.SelectedIndex, GraphicIndexUpDown.Value)
    End Sub

    Private Sub GraphicTypeCombo_SelectedIndexChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GraphicTypeComboBox.SelectedIndexChanged
        GraphicIndexUpDown.Maximum = _dm.ItemImages.SmallItems(GraphicTypeComboBox.SelectedIndex).Length - 1
        LoadImages(GraphicTypeComboBox.SelectedIndex, GraphicIndexUpDown.Value)
        ImageListBox.DataSource = _dm.ItemImages.BigItems(GraphicTypeComboBox.SelectedIndex)
    End Sub

    Private Sub ImageListBox_MeasureItem(ByVal sender As Object, ByVal e As System.Windows.Forms.MeasureItemEventArgs) Handles ImageListBox.MeasureItem
        If e.Index < 0 Then Return

        Dim img As Image = CType(ImageListBox.Items(e.Index), Image)
        Dim hgt As Single = Math.Max(img.Height, e.Graphics.MeasureString("1", ImageListBox.Font).Height) * 1.1
        e.ItemHeight = hgt
        e.ItemWidth = ImageListBox.Width
    End Sub

    Private Sub ImageListBox_SelectedIndexChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ImageListBox.SelectedIndexChanged
        GraphicIndexUpDown.Value = ImageListBox.SelectedIndex
    End Sub

    Private Sub ImageListBox_DrawItem(ByVal sender As Object, ByVal e As System.Windows.Forms.DrawItemEventArgs) Handles ImageListBox.DrawItem
        Dim g As Graphics = e.Graphics
        If e.State = DrawItemState.Selected Then
            'fill the background of the item 
            g.FillRectangle(Brushes.Blue, e.Bounds)
            'draw the image from the image list control, offset it by 5 pixels and makes sure it's centered vertically 
            Dim myImage As Bitmap = CType(ImageListBox.Items(e.Index), Bitmap)
            g.DrawImage(myImage, 5, e.Bounds.Top + (e.Bounds.Height - myImage.Height) \ 2)
        Else
            'this block does the same thing as above but uses different colors to represent the different state. 
            Dim myImage As Bitmap = CType(ImageListBox.Items(e.Index), Bitmap)
            g.FillRectangle(Brushes.White, e.Bounds)
            g.DrawImage(myImage, 5, e.Bounds.Top + (e.Bounds.Height - myImage.Height) \ 2)
        End If
    End Sub
#End Region

#Region " Descriptions Tab "
    Private Sub DescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DescriptionTextBox.TextChanged
        DescriptionCharsLeftLabel.Text = DescriptionTextBox.MaxLength - DescriptionTextBox.TextLength
    End Sub

    Private Sub BRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles BRDescriptionTextBox.TextChanged
        BRDescriptionCharsLeftLabel.Text = BRDescriptionTextBox.MaxLength - BRDescriptionTextBox.TextLength
    End Sub

    Private Sub GermanDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GermanDescriptionTextBox.TextChanged
        GermanDescriptionCharsLeftLabel.Text = GermanDescriptionTextBox.MaxLength - GermanDescriptionTextBox.TextLength
    End Sub

    Private Sub GermanBRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles GermanBRDescriptionTextBox.TextChanged
        GermanBRDescriptionCharsLeftLabel.Text = GermanBRDescriptionTextBox.MaxLength - GermanBRDescriptionTextBox.TextLength
    End Sub

    Private Sub RussianDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RussianDescriptionTextBox.TextChanged
        RussianDescriptionCharsLeftLabel.Text = RussianDescriptionTextBox.MaxLength - RussianDescriptionTextBox.TextLength
    End Sub

    Private Sub RussianBRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles RussianBRDescriptionTextBox.TextChanged
        RussianBRDescriptionCharsLeftLabel.Text = RussianBRDescriptionTextBox.MaxLength - RussianBRDescriptionTextBox.TextLength
    End Sub

    Private Sub PolishDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles PolishDescriptionTextBox.TextChanged
        PolishDescriptionCharsLeftLabel.Text = PolishDescriptionTextBox.MaxLength - PolishDescriptionTextBox.TextLength
    End Sub

    Private Sub PolishBRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles PolishBRDescriptionTextBox.TextChanged
        PolishBRDescriptionCharsLeftLabel.Text = PolishBRDescriptionTextBox.MaxLength - PolishBRDescriptionTextBox.TextLength
    End Sub

    Private Sub FrenchDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles FrenchDescriptionTextBox.TextChanged
        FrenchDescriptionCharsLeftLabel.Text = FrenchDescriptionTextBox.MaxLength - FrenchDescriptionTextBox.TextLength
    End Sub

    Private Sub FrenchBRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles FrenchBRDescriptionTextBox.TextChanged
        FrenchBRDescriptionCharsLeftLabel.Text = FrenchBRDescriptionTextBox.MaxLength - FrenchBRDescriptionTextBox.TextLength
    End Sub

    Private Sub ItalianDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ItalianDescriptionTextBox.TextChanged
        ItalianDescriptionCharsLeftLabel.Text = ItalianDescriptionTextBox.MaxLength - ItalianDescriptionTextBox.TextLength
    End Sub

    Private Sub ItalianBRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ItalianBRDescriptionTextBox.TextChanged
        ItalianBRDescriptionCharsLeftLabel.Text = ItalianBRDescriptionTextBox.MaxLength - ItalianBRDescriptionTextBox.TextLength
    End Sub

    Private Sub DutchDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DutchDescriptionTextBox.TextChanged
        DutchDescriptionCharsLeftLabel.Text = DutchDescriptionTextBox.MaxLength - DutchDescriptionTextBox.TextLength
    End Sub

    Private Sub DutchBRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DutchBRDescriptionTextBox.TextChanged
        DutchBRDescriptionCharsLeftLabel.Text = DutchBRDescriptionTextBox.MaxLength - DutchBRDescriptionTextBox.TextLength
    End Sub

    Private Sub ChineseDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ChineseDescriptionTextBox.TextChanged
        ChineseDescriptionCharsLeftLabel.Text = ChineseDescriptionTextBox.MaxLength - ChineseDescriptionTextBox.TextLength
    End Sub

    Private Sub ChineseBRDescriptionTextBox_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ChineseBRDescriptionTextBox.TextChanged
        ChineseBRDescriptionCharsLeftLabel.Text = ChineseBRDescriptionTextBox.MaxLength - ChineseBRDescriptionTextBox.TextLength
    End Sub
#End Region

#Region " Weapons Tab "
    Private Sub ubShotsPer4TurnsUpDown_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ubShotsPer4TurnsUpDown.ValueChanged
        If ubShotsPer4TurnsUpDown.Value > 0 Then
            ' RoWa21: Generic formular for any AP system
            Dim maxAPs As Integer = IniFile.APMaximum()

            ' If not set in the "XMLEditorInit.xml", then set it here to default 100 APs
            If maxAPs <= 0 Then
                maxAPs = 100
            End If

            Dim multiplicator As Integer = Math.Round(maxAPs / 25, 0)
            Dim val As Integer = Math.Round((2 * multiplicator * 80 * 100) / ((100 + 80) * ubShotsPer4TurnsUpDown.Value), 0)

            ' RoWa21: This is the simplified old formular for the 25 AP system
            'Dim val As Integer = Math.Round(89 / ubShotsPer4TurnsUpDown.Value, 0)
            Me.APsShots4TurnsLabel.Text = "= " & val & " APs"
        Else
            Me.APsShots4TurnsLabel.Text = ""
        End If
    End Sub

    Private Sub ubReadyTimeUpDown_ValueChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ubReadyTimeUpDown.ValueChanged
        ' RoWa21: Generic formular for any AP system
        Dim maxAPs As Integer = IniFile.APMaximum()

        ' If not set in the "XMLEditorInit.xml", then set it here to default 25 APs
        If maxAPs <= 0 Then
            maxAPs = 25
        End If

        Dim val As Integer = Math.Round((ubReadyTimeUpDown.Value * maxAPs) / 100, 0)

        ' RoWa21: Simplified old formular for the 25 AP system
        Me.APsReadyLabel.Text = "= " & val & " APs"
    End Sub
#End Region

#Region " Data Binding "
    Protected Overrides Function CommitData() As Boolean
        Try
            Dim newID As Integer = _view(0)(Tables.Items.Fields.ID)
            Dim oldID As Integer = _view(0).Row(Tables.Items.Fields.ID, DataRowVersion.Current)
            Dim otherItem As DataRow = _view.Table.Rows.Find(newID)

            _view(0)(Tables.Items.Fields.ItemImage) = BigItemImage.Image
            _view(0)(Tables.Items.Fields.GraphicIndex) = GraphicIndexUpDown.Value
            _view(0)(Tables.Items.Fields.GraphicType) = GraphicTypeComboBox.SelectedIndex

            If otherItem IsNot Nothing AndAlso otherItem IsNot _view(0).Row Then
                If MessageBox.Show("The Item ID you have entered is already being used by """ & otherItem(Tables.Items.Fields.Name) & """.  Do you want to swap IDs?", "Swap IDs", MessageBoxButtons.YesNo, MessageBoxIcon.Question, MessageBoxDefaultButton.Button1) = System.Windows.Forms.DialogResult.Yes Then
                    'swap ids
                    otherItem(Tables.Items.Fields.ID) = -1
                    _view(0).EndEdit()
                    otherItem(Tables.Items.Fields.ID) = oldID
                    _id = newID
                    _view.RowFilter = Tables.Items.Fields.ID & "=" & _id
                    Me.Text = String.Format(DisplayText.ItemDataFormText, _dm.Name, _id, _view(0)(Tables.Items.Fields.Name))
                    SaveInventoryData()
                Else
                    ErrorHandler.ShowWarning("Please enter a different ID value.", MessageBoxIcon.Exclamation)
                    Return False
                End If
            Else
                _view(0).EndEdit()
                _id = newID
                _view.RowFilter = Tables.Items.Fields.ID & "=" & _id
                Me.Text = String.Format(DisplayText.ItemDataFormText, _dm.Name, _id, _view(0)(Tables.Items.Fields.Name))
                SaveInventoryData()
            End If

            'Saving Itemflags
            Dim TempChecklistBox As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox64").Controls.Item("ItemFlagsCheckedList")
            _view(0)("Bloodbag") = TempChecklistBox.GetItemChecked(0)
            _view(0)("Manpad") = TempChecklistBox.GetItemChecked(1)
            _view(0)("Beartrap") = TempChecklistBox.GetItemChecked(2)
            _view(0)("Camera") = TempChecklistBox.GetItemChecked(3)
            _view(0)("Waterdrum") = TempChecklistBox.GetItemChecked(4)
            _view(0)("BloodcatMeat") = TempChecklistBox.GetItemChecked(5)
            _view(0)("CowMeat") = TempChecklistBox.GetItemChecked(6)
            _view(0)("Beltfed") = TempChecklistBox.GetItemChecked(7)
            _view(0)("Ammobelt") = TempChecklistBox.GetItemChecked(8)
            _view(0)("AmmobeltVest") = TempChecklistBox.GetItemChecked(9)
            _view(0)("CamoRemoval") = TempChecklistBox.GetItemChecked(10)
            _view(0)("Cleaningkit") = TempChecklistBox.GetItemChecked(11)
            _view(0)("AttentionItem") = TempChecklistBox.GetItemChecked(12)
            _view(0)("Garotte") = TempChecklistBox.GetItemChecked(13)
            _view(0)("Covert") = TempChecklistBox.GetItemChecked(14)
            _view(0)("Corpse") = TempChecklistBox.GetItemChecked(15)
            _view(0)("BloodcatSkin") = TempChecklistBox.GetItemChecked(16)
            _view(0)("NoMetalDetection") = TempChecklistBox.GetItemChecked(17)
            _view(0)("JumpGrenade") = TempChecklistBox.GetItemChecked(18)
            _view(0)("Handcuffs") = TempChecklistBox.GetItemChecked(19)
            _view(0)("Taser") = TempChecklistBox.GetItemChecked(20)
            _view(0)("ScubaBottle") = TempChecklistBox.GetItemChecked(21)
            _view(0)("ScubaMask") = TempChecklistBox.GetItemChecked(22)
            _view(0)("ScubaFins") = TempChecklistBox.GetItemChecked(23)
            _view(0)("TripwireRoll") = TempChecklistBox.GetItemChecked(24)
            _view(0)("Radioset") = TempChecklistBox.GetItemChecked(25)
            _view(0)("SignalShell") = TempChecklistBox.GetItemChecked(26)
            _view(0)("Soda") = TempChecklistBox.GetItemChecked(27)
            _view(0)("RoofcollapseItem") = TempChecklistBox.GetItemChecked(28)
            _view(0)("DiseaseprotectionFace") = TempChecklistBox.GetItemChecked(29)
            _view(0)("DiseaseprotectionHand") = TempChecklistBox.GetItemChecked(30)
            _view(0)("LBEexplosionproof") = TempChecklistBox.GetItemChecked(31)
            _view(0)("EmptyBloodbag") = TempChecklistBox.GetItemChecked(32)
            _view(0)("MedicalSplint") = TempChecklistBox.GetItemChecked(33)
            _view(0)("Damageable") = TempChecklistBox.GetItemChecked(34)
            _view(0)("Repairable") = TempChecklistBox.GetItemChecked(35)
            _view(0)("WaterDamages") = TempChecklistBox.GetItemChecked(36)
            _view(0)("Metal") = TempChecklistBox.GetItemChecked(37)
            _view(0)("Sinks") = TempChecklistBox.GetItemChecked(38)
            _view(0)("ShowStatus") = TempChecklistBox.GetItemChecked(39)
            _view(0)("HiddenAddon") = TempChecklistBox.GetItemChecked(40)
            _view(0)("TwoHanded") = TempChecklistBox.GetItemChecked(41)
            _view(0)("NotBuyable") = TempChecklistBox.GetItemChecked(42)
            _view(0)("Attachment") = TempChecklistBox.GetItemChecked(43)
            _view(0)("HiddenAttachment") = TempChecklistBox.GetItemChecked(44)
            _view(0)("BigGunList") = TempChecklistBox.GetItemChecked(45)
            _view(0)("NotInEditor") = TempChecklistBox.GetItemChecked(46)
            _view(0)("DefaultUndroppable") = TempChecklistBox.GetItemChecked(47)
            _view(0)("Unaerodynamic") = TempChecklistBox.GetItemChecked(48)
            _view(0)("Electronic") = TempChecklistBox.GetItemChecked(49)
            _view(0)("Cannon") = TempChecklistBox.GetItemChecked(50)
            _view(0)("RocketRifle") = TempChecklistBox.GetItemChecked(51)
            _view(0)("FingerPrintID") = TempChecklistBox.GetItemChecked(52)
            _view(0)("MetalDetector") = TempChecklistBox.GetItemChecked(53)
            _view(0)("GasMask") = TempChecklistBox.GetItemChecked(54)
            _view(0)("LockBomb") = TempChecklistBox.GetItemChecked(55)
            _view(0)("Flare") = TempChecklistBox.GetItemChecked(56)
            _view(0)("GrenadeLauncher") = TempChecklistBox.GetItemChecked(57)
            _view(0)("Mortar") = TempChecklistBox.GetItemChecked(58)
            _view(0)("Duckbill") = TempChecklistBox.GetItemChecked(59)
            _view(0)("Detonator") = TempChecklistBox.GetItemChecked(60)
            _view(0)("RemoteDetonator") = TempChecklistBox.GetItemChecked(61)
            _view(0)("HideMuzzleFlash") = TempChecklistBox.GetItemChecked(62)
            _view(0)("RocketLauncher") = TempChecklistBox.GetItemChecked(63)

            'ItemFlag2
            TempChecklistBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox67").Controls.Item("ItemFlags2CheckedList")
            _view(0)("SingleShotRocketLauncher") = TempChecklistBox.GetItemChecked(0)
            _view(0)("BrassKnuckles") = TempChecklistBox.GetItemChecked(1)
            _view(0)("Crowbar") = TempChecklistBox.GetItemChecked(2)
            _view(0)("GLGrenade") = TempChecklistBox.GetItemChecked(3)
            _view(0)("FlakJacket") = TempChecklistBox.GetItemChecked(4)
            _view(0)("LeatherJacket") = TempChecklistBox.GetItemChecked(5)
            _view(0)("Batteries") = TempChecklistBox.GetItemChecked(6)
            _view(0)("NeedsBatteries") = TempChecklistBox.GetItemChecked(7)
            _view(0)("XRay") = TempChecklistBox.GetItemChecked(8)
            _view(0)("WireCutters") = TempChecklistBox.GetItemChecked(9)
            _view(0)("Toolkit") = TempChecklistBox.GetItemChecked(10)
            _view(0)("FirstAidKit") = TempChecklistBox.GetItemChecked(11)
            _view(0)("MedicalKit") = TempChecklistBox.GetItemChecked(12)
            _view(0)("Canteen") = TempChecklistBox.GetItemChecked(13)
            _view(0)("Jar") = TempChecklistBox.GetItemChecked(14)
            _view(0)("CanAndString") = TempChecklistBox.GetItemChecked(15)
            _view(0)("Marbles") = TempChecklistBox.GetItemChecked(16)
            _view(0)("Walkman") = TempChecklistBox.GetItemChecked(17)
            _view(0)("RemoteTrigger") = TempChecklistBox.GetItemChecked(18)
            _view(0)("RobotRemoteControl") = TempChecklistBox.GetItemChecked(19)
            _view(0)("CamouflageKit") = TempChecklistBox.GetItemChecked(20)
            _view(0)("LocksmithKit") = TempChecklistBox.GetItemChecked(21)
            _view(0)("Mine") = TempChecklistBox.GetItemChecked(22)
            _view(0)("AntitankMine") = TempChecklistBox.GetItemChecked(23)
            _view(0)("Hardware") = TempChecklistBox.GetItemChecked(24)
            _view(0)("Medical") = TempChecklistBox.GetItemChecked(25)
            _view(0)("GasCan") = TempChecklistBox.GetItemChecked(26)
            _view(0)("ContainsLiquid") = TempChecklistBox.GetItemChecked(27)
            _view(0)("Rock") = TempChecklistBox.GetItemChecked(28)
            _view(0)("ThermalOptics") = TempChecklistBox.GetItemChecked(29)
            _view(0)("SciFi") = TempChecklistBox.GetItemChecked(30)
            _view(0)("NewInv") = TempChecklistBox.GetItemChecked(31)
            _view(0)("DiseaseSystemExclusive") = TempChecklistBox.GetItemChecked(32)
            _view(0)("Barrel") = TempChecklistBox.GetItemChecked(33)
            _view(0)("TripWireActivation") = TempChecklistBox.GetItemChecked(34)
            _view(0)("TripWire") = TempChecklistBox.GetItemChecked(35)
            _view(0)("Directional") = TempChecklistBox.GetItemChecked(36)
            _view(0)("BlockIronSight") = TempChecklistBox.GetItemChecked(37)
            _view(0)("AllowClimbing") = TempChecklistBox.GetItemChecked(38)
            _view(0)("Cigarette") = TempChecklistBox.GetItemChecked(39)
            _view(0)("ProvidesRobotCamo") = TempChecklistBox.GetItemChecked(40)
            _view(0)("ProvidesRobotNightVision") = TempChecklistBox.GetItemChecked(41)
            _view(0)("ProvidesRobotLaserBonus") = TempChecklistBox.GetItemChecked(42)


            ''Saving Drugs
            'Dim TempDrugFlags As UInt32 = 0
            'Dim TempDrugChecklistBox As CheckedListBox = ItemTab.TabPages("FlagsTab").Controls.Item("GroupBox65").Controls.Item("DrugsCheckedList")
            'For i As Integer = 0 To 31
            '    If TempDrugChecklistBox.GetItemChecked(i) Then
            '        TempDrugFlags += 2 ^ i
            '    End If
            'Next

            '_view(0)("DrugType") = TempDrugFlags

            _view(0).Row.AcceptChanges()
            AcceptGridChanges(AttachmentGrid)
            AcceptGridChanges(AttachToGrid)
            AcceptGridChanges(IncompatibleAttachmentGrid)
            AcceptGridChanges(LaunchableGrid)
            AcceptGridChanges(LauncherGrid)
            AcceptGridChanges(AttachmentInfoGrid)
            AcceptGridChanges(CompatibleFaceItemGrid)
            Return True
        Catch ex As ConstraintException
            ErrorHandler.ShowError("One or more values must be unique. Please enter a different value(s).", MessageBoxIcon.Exclamation, ex)
        Catch ex As Exception
            ErrorHandler.ShowError(ex)
        End Try

    End Function

    Protected Overrides Sub DoCancelData()
        CancelGridChanges(AttachmentGrid)
        CancelGridChanges(AttachToGrid)
        CancelGridChanges(IncompatibleAttachmentGrid)
        CancelGridChanges(LaunchableGrid)
        CancelGridChanges(LauncherGrid)
        CancelGridChanges(AttachmentInfoGrid)
        CancelGridChanges(CompatibleFaceItemGrid)
    End Sub
#End Region

#Region " Buttons "

    Protected Overrides Sub ApplyButtonClicked()
        GraphicIndexUpDown.Value = _view(0)(Tables.Items.Fields.GraphicIndex)
    End Sub

    Private Sub ChangeClassButton_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ChangeClassButton.Click
        Dim frm As New ChangeClassForm(_dm, _view, Me)
        frm.ShowDialog()
    End Sub

#End Region

#Region " Grids "
    Protected Sub SetupGrid(ByVal grid As DataGridView, ByVal tableName As String, Optional ByVal itemIndexField As String = Nothing)
        Dim t As DataTable = _dm.Database.Table(tableName)
        Dim rowFilter As String = Nothing
        If itemIndexField IsNot Nothing Then
            rowFilter = itemIndexField & "=" & _id
        End If

        InitializeGrid(_dm.Database, grid, New DataView(t, rowFilter, "", DataViewRowState.CurrentRows), , True)

        grid.Tag = itemIndexField
        grid.Columns(itemIndexField).Visible = False

        AddHandler grid.DefaultValuesNeeded, AddressOf Grid_DefaultValuesNeeded
    End Sub

    Protected Sub Grid_DefaultValuesNeeded(ByVal sender As Object, ByVal e As System.Windows.Forms.DataGridViewRowEventArgs)
        Dim grid As DataGridView = DirectCast(sender, DataGridView)
        Dim itemIndexField As String = grid.Tag
        e.Row.Cells(itemIndexField).Value = _id

        'set primary key for single key grids, like the attachmentinfo one
        Dim v As DataView = DirectCast(grid.DataSource, DataView)
        If v.Table.PrimaryKey.Length = 1 Then
            Dim key As String = v.Table.PrimaryKey(0).ColumnName
            Dim val As Decimal = _dm.Database.GetNextPrimaryKeyValue(v.Table)
            e.Row.Cells(key).Value = val
        End If
    End Sub

    Protected Sub CancelGridChanges(ByVal grid As DataGridView)
        Dim v As DataView = DirectCast(grid.DataSource, DataView)
        v.RowStateFilter = DataViewRowState.Added Or DataViewRowState.ModifiedCurrent Or DataViewRowState.Deleted
        For i As Integer = v.Count - 1 To 0 Step -1
            If v(i).Row.RowState <> DataRowState.Detached Then v(i).Row.RejectChanges()
        Next
        v.RowStateFilter = DataViewRowState.CurrentRows
    End Sub

    Protected Sub AcceptGridChanges(ByVal grid As DataGridView)
        Dim v As DataView = DirectCast(grid.DataSource, DataView)
        v.RowStateFilter = DataViewRowState.Added Or DataViewRowState.ModifiedCurrent Or DataViewRowState.Deleted
        For i As Integer = v.Count - 1 To 0 Step -1
            v(i).Row.AcceptChanges()
        Next
        v.RowStateFilter = DataViewRowState.CurrentRows
    End Sub

    Private Sub AttachToGrid_RowHeaderMouseDoubleClick(ByVal sender As Object, ByVal e As System.Windows.Forms.DataGridViewCellMouseEventArgs) Handles AttachToGrid.RowHeaderMouseDoubleClick
        GridOpenItem(AttachToGrid, e.RowIndex, Tables.Attachments.Fields.ItemID)
    End Sub

    Private Sub AttachmentGrid_RowHeaderMouseDoubleClick(ByVal sender As Object, ByVal e As System.Windows.Forms.DataGridViewCellMouseEventArgs) Handles AttachmentGrid.RowHeaderMouseDoubleClick
        GridOpenItem(AttachmentGrid, e.RowIndex, Tables.Attachments.Fields.AttachmentID)
    End Sub

    Private Sub IncompatibleAttachmentGrid_RowHeaderMouseDoubleClick(ByVal sender As Object, ByVal e As System.Windows.Forms.DataGridViewCellMouseEventArgs) Handles IncompatibleAttachmentGrid.RowHeaderMouseDoubleClick
        GridOpenItem(IncompatibleAttachmentGrid, e.RowIndex, Tables.IncompatibleAttachments.Fields.IncompatibleItemID)
    End Sub

    Private Sub LaunchableGrid_RowHeaderMouseDoubleClick(ByVal sender As Object, ByVal e As System.Windows.Forms.DataGridViewCellMouseEventArgs) Handles LaunchableGrid.RowHeaderMouseDoubleClick
        GridOpenItem(LaunchableGrid, e.RowIndex, Tables.Launchables.Fields.LaunchableID)
    End Sub

    Private Sub LauncherGrid_RowHeaderMouseDoubleClick(ByVal sender As Object, ByVal e As System.Windows.Forms.DataGridViewCellMouseEventArgs) Handles LauncherGrid.RowHeaderMouseDoubleClick
        GridOpenItem(LauncherGrid, e.RowIndex, Tables.Launchables.Fields.ItemID)
    End Sub

    Private Sub CompatibleFaceItemGrid_RowHeaderMouseDoubleClick(ByVal sender As Object, ByVal e As System.Windows.Forms.DataGridViewCellMouseEventArgs) Handles CompatibleFaceItemGrid.RowHeaderMouseDoubleClick
        GridOpenItem(CompatibleFaceItemGrid, e.RowIndex, Tables.CompatibleFaceItems.Fields.CompatibleItemID)
    End Sub

    Private Sub GridOpenItem(ByVal grid As DataGridView, ByVal rowIndex As Integer, ByVal itemIDFieldName As String)
        Dim name As String = Nothing
        Dim id As Integer = grid.Rows(rowIndex).Cells(itemIDFieldName).Value
        Dim r As DataRow = _dm.Database.Table(Tables.Items.Name).Rows.Find(id)
        If r IsNot Nothing Then
            name = r(Tables.Items.Fields.Name)
            Open(_dm, id, name)
        End If
    End Sub
#End Region

#Region " Tabs "
    Protected Sub DisplayTabs()
        With ItemTab.TabPages
            Select Case _view(0)(Tables.Items.Fields.ItemClass)
                Case ItemClass.Ammo
                    .Remove(WeaponPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(AttachmentPage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.Armour
                    .Remove(WeaponPage)
                    .Remove(AmmoPage)
                    .Remove(ExplosivePage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.Bomb, ItemClass.Grenade
                    .Remove(WeaponPage)
                    .Remove(ArmourPage)
                    .Remove(AmmoPage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.Gun
                    WeaponTab.TabPages.Remove(LauncherPage)
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    '.Remove(AttachmentDataPage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                Case ItemClass.Launcher
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                Case ItemClass.Knife, ItemClass.ThrowingKnife, ItemClass.Thrown, ItemClass.Tentacle, ItemClass.Punch
                    WeaponTab.TabPages.Remove(LauncherPage)
                    WeaponTab.TabPages.Remove(GunPage)
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.Face
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(WeaponPage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.Kit, ItemClass.MedKit, ItemClass.Misc
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(WeaponPage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.Key
                    .Remove(AttachmentPage)
                    .Remove(AttachmentDataPage)
                    .Remove(NCTHPage)
                    .Remove(BonusesPage)
                    .Remove(AbilitiesPage)
                    .Remove(WeaponPage)
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.LBE
                    '.Remove(AttachmentPage) 'Commenting this out for MOLLE to work properly.
                    .Remove(AttachmentDataPage)
                    .Remove(NCTHPage)
                    .Remove(BonusesPage)
                    .Remove(WeaponPage)
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(FacePage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
                Case ItemClass.RandomItem
                    .Remove(NCTHPage)
                    .Remove(BonusesPage)
                    .Remove(AttachmentPage)
                    .Remove(AttachmentDataPage)
                    .Remove(ExplosivePage)
                    .Remove(InventoryPage)
                    .Remove(OverheatingTabPage)
                    .Remove(FacePage)
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(WeaponPage)
                    .Remove(LBEPage)
                Case Else 'ItemClass.None, ItemClass.Money
                    .Remove(AttachmentPage)
                    .Remove(AttachmentDataPage)
                    .Remove(NCTHPage)
                    .Remove(BonusesPage)
                    .Remove(AbilitiesPage)
                    .Remove(InventoryPage)
                    .Remove(WeaponPage)
                    .Remove(AmmoPage)
                    .Remove(ArmourPage)
                    .Remove(ExplosivePage)
                    .Remove(FacePage)
                    .Remove(LBEPage)
                    OverheatingTabPage.Controls.Remove(WeaponTemperatureGroupBox)
            End Select
        End With
    End Sub
#End Region

#Region " Shared methods "
    Public Shared Sub Open(manager As DataManager, ByVal id As Integer, ByVal name As String)
        Dim formText As String = String.Format(DisplayText.ItemDataFormText, manager.Name, id, name)
        If Not MainWindow.FormOpen(formText) Then
            Dim frm As New ItemDataForm(manager, id, formText)
            MainWindow.ShowForm(frm)
        End If
    End Sub
#End Region

#Region " Shopkeeper Inventories "

    Private Sub InventoryCheckBox_CheckedChanged(ByVal sender As Object, ByVal e As System.EventArgs)
        Dim cb As CheckBox = DirectCast(sender, CheckBox)
        ShopkeepersPanel.Controls(cb.Text & "UpDown").Enabled = cb.Checked
    End Sub

    Protected Sub LoadInventoryData()
        LoadShopkeeperData(ShopKeepers.Alberto)
        LoadShopkeeperData(ShopKeepers.Arnie)
        LoadShopkeeperData(ShopKeepers.Carlo)
        LoadShopkeeperData(ShopKeepers.Devin)
        LoadShopkeeperData(ShopKeepers.Elgin)
        LoadShopkeeperData(ShopKeepers.Frank)
        LoadShopkeeperData(ShopKeepers.Franz)
        LoadShopkeeperData(ShopKeepers.Fredo)
        LoadShopkeeperData(ShopKeepers.Gabby)
        LoadShopkeeperData(ShopKeepers.Herve)
        LoadShopkeeperData(ShopKeepers.Howard)
        LoadShopkeeperData(ShopKeepers.Jake)
        LoadShopkeeperData(ShopKeepers.Keith)
        LoadShopkeeperData(ShopKeepers.Manny)
        LoadShopkeeperData(ShopKeepers.Mickey)
        LoadShopkeeperData(ShopKeepers.Perko)
        LoadShopkeeperData(ShopKeepers.Peter)
        LoadShopkeeperData(ShopKeepers.Sam)
        LoadShopkeeperData(ShopKeepers.Tony)
    End Sub

    Protected Sub SaveInventoryData()
        SaveShopkeeperData(ShopKeepers.Alberto)
        SaveShopkeeperData(ShopKeepers.Arnie)
        SaveShopkeeperData(ShopKeepers.Carlo)
        SaveShopkeeperData(ShopKeepers.Devin)
        SaveShopkeeperData(ShopKeepers.Elgin)
        SaveShopkeeperData(ShopKeepers.Frank)
        SaveShopkeeperData(ShopKeepers.Franz)
        SaveShopkeeperData(ShopKeepers.Fredo)
        SaveShopkeeperData(ShopKeepers.Gabby)
        SaveShopkeeperData(ShopKeepers.Herve)
        SaveShopkeeperData(ShopKeepers.Howard)
        SaveShopkeeperData(ShopKeepers.Jake)
        SaveShopkeeperData(ShopKeepers.Keith)
        SaveShopkeeperData(ShopKeepers.Manny)
        SaveShopkeeperData(ShopKeepers.Mickey)
        SaveShopkeeperData(ShopKeepers.Perko)
        SaveShopkeeperData(ShopKeepers.Peter)
        SaveShopkeeperData(ShopKeepers.Sam)
        SaveShopkeeperData(ShopKeepers.Tony)
    End Sub

    Protected Sub LoadShopkeeperData(ByVal shopKeeperName As String)
        Dim rows() As DataRow = _dm.Database.Table(shopKeeperName & Tables.Inventory).Select(Tables.InventoryTableFields.ItemID & "=" & _id)
        Dim cb As CheckBox = DirectCast(ShopkeepersPanel.Controls(shopKeeperName & "CheckBox"), CheckBox)
        Dim ud As NumericUpDown = DirectCast(ShopkeepersPanel.Controls(shopKeeperName & "UpDown"), NumericUpDown)

        If rows.Length > 0 Then
            cb.Checked = True
            ud.Value = rows(0)(Tables.InventoryTableFields.Quantity)
            ud.Enabled = True
        Else
            cb.Checked = False
            ud.Value = _dm.Database.Table(shopKeeperName & Tables.Inventory).Columns(Tables.InventoryTableFields.Quantity).DefaultValue
            ud.Enabled = False
        End If

        AddHandler cb.CheckedChanged, AddressOf InventoryCheckBox_CheckedChanged
    End Sub

    Protected Sub SaveShopkeeperData(ByVal shopKeeperName As String)
        Dim rows() As DataRow = _dm.Database.Table(shopKeeperName & Tables.Inventory).Select(Tables.InventoryTableFields.ItemID & "=" & _id)
        Dim cb As CheckBox = DirectCast(ShopkeepersPanel.Controls(shopKeeperName & "CheckBox"), CheckBox)
        Dim ud As NumericUpDown = DirectCast(ShopkeepersPanel.Controls(shopKeeperName & "UpDown"), NumericUpDown)

        If cb.Checked Then
            If rows.Length > 0 Then 'modify
                rows(0)(Tables.InventoryTableFields.Quantity) = ud.Value
            Else 'new
                Dim row As DataRow = _dm.Database.NewRow(shopKeeperName & Tables.Inventory)
                'key value is set automatically
                row(Tables.InventoryTableFields.ItemID) = _id
                row(Tables.InventoryTableFields.Quantity) = ud.Value
            End If
        Else 'delete
            If rows.Length > 0 Then rows(0).Delete()
        End If
    End Sub
#End Region

#Region " Enable / disable LBE Pockets "

    Private Sub DisableAllPockets()
        ' Disable pockets
        lbePocketIndex1ComboBox.Enabled = False
        lbePocketIndex2ComboBox.Enabled = False
        lbePocketIndex3ComboBox.Enabled = False
        lbePocketIndex4ComboBox.Enabled = False
        lbePocketIndex5ComboBox.Enabled = False
        lbePocketIndex6ComboBox.Enabled = False
        lbePocketIndex7ComboBox.Enabled = False
        lbePocketIndex8ComboBox.Enabled = False
        lbePocketIndex9ComboBox.Enabled = False
        lbePocketIndex10ComboBox.Enabled = False
        lbePocketIndex11ComboBox.Enabled = False
        lbePocketIndex12ComboBox.Enabled = False

        ' Set the values of the disabled pockets to "Nothing"
        lbePocketIndex1ComboBox.SelectedValue = 0
        lbePocketIndex2ComboBox.SelectedValue = 0
        lbePocketIndex3ComboBox.SelectedValue = 0
        lbePocketIndex4ComboBox.SelectedValue = 0
        lbePocketIndex5ComboBox.SelectedValue = 0
        lbePocketIndex6ComboBox.SelectedValue = 0
        lbePocketIndex7ComboBox.SelectedValue = 0
        lbePocketIndex8ComboBox.SelectedValue = 0
        lbePocketIndex9ComboBox.SelectedValue = 0
        lbePocketIndex10ComboBox.SelectedValue = 0
        lbePocketIndex11ComboBox.SelectedValue = 0
        lbePocketIndex12ComboBox.SelectedValue = 0
    End Sub

    Private Sub EnableThighPackPockets()
        ' Enable pockets
        lbePocketIndex1ComboBox.Enabled = True
        lbePocketIndex2ComboBox.Enabled = True
        lbePocketIndex3ComboBox.Enabled = True
        lbePocketIndex4ComboBox.Enabled = True
        lbePocketIndex5ComboBox.Enabled = True

        ' Disable pockets
        lbePocketIndex6ComboBox.Enabled = False
        lbePocketIndex7ComboBox.Enabled = False
        lbePocketIndex8ComboBox.Enabled = False
        lbePocketIndex9ComboBox.Enabled = False
        lbePocketIndex10ComboBox.Enabled = False
        lbePocketIndex11ComboBox.Enabled = False
        lbePocketIndex12ComboBox.Enabled = False

        ' Set the values of the disabled pockets to "Nothing"
        lbePocketIndex6ComboBox.SelectedValue = 0
        lbePocketIndex7ComboBox.SelectedValue = 0
        lbePocketIndex8ComboBox.SelectedValue = 0
        lbePocketIndex9ComboBox.SelectedValue = 0
        lbePocketIndex10ComboBox.SelectedValue = 0
        lbePocketIndex11ComboBox.SelectedValue = 0
        lbePocketIndex12ComboBox.SelectedValue = 0
    End Sub

    Private Sub EnableVestPackPockets()
        ' Enable pockets
        lbePocketIndex1ComboBox.Enabled = True
        lbePocketIndex2ComboBox.Enabled = True
        lbePocketIndex3ComboBox.Enabled = True
        lbePocketIndex4ComboBox.Enabled = True
        lbePocketIndex5ComboBox.Enabled = True
        lbePocketIndex6ComboBox.Enabled = True
        lbePocketIndex7ComboBox.Enabled = True
        lbePocketIndex8ComboBox.Enabled = True
        lbePocketIndex9ComboBox.Enabled = True
        lbePocketIndex10ComboBox.Enabled = True
        lbePocketIndex11ComboBox.Enabled = True
        lbePocketIndex12ComboBox.Enabled = True
    End Sub

    Private Sub EnableBackPackPockets()
        ' Enable pockets
        lbePocketIndex1ComboBox.Enabled = True
        lbePocketIndex2ComboBox.Enabled = True
        lbePocketIndex3ComboBox.Enabled = True
        lbePocketIndex4ComboBox.Enabled = True
        lbePocketIndex5ComboBox.Enabled = True
        lbePocketIndex6ComboBox.Enabled = True
        lbePocketIndex7ComboBox.Enabled = True
        lbePocketIndex8ComboBox.Enabled = True
        lbePocketIndex9ComboBox.Enabled = True
        lbePocketIndex10ComboBox.Enabled = True
        lbePocketIndex11ComboBox.Enabled = True
        lbePocketIndex12ComboBox.Enabled = True
    End Sub

    Private Sub EnableCombatPackPockets()
        ' Enable pockets
        lbePocketIndex1ComboBox.Enabled = True
        lbePocketIndex2ComboBox.Enabled = True
        lbePocketIndex3ComboBox.Enabled = True
        lbePocketIndex4ComboBox.Enabled = True
        lbePocketIndex5ComboBox.Enabled = True
        lbePocketIndex6ComboBox.Enabled = True
        lbePocketIndex7ComboBox.Enabled = True

        ' Disable pockets
        lbePocketIndex8ComboBox.Enabled = False
        lbePocketIndex9ComboBox.Enabled = False
        lbePocketIndex10ComboBox.Enabled = False
        lbePocketIndex11ComboBox.Enabled = False
        lbePocketIndex12ComboBox.Enabled = False

        ' Set the values of the disabled pockets to "Nothing"
        lbePocketIndex8ComboBox.SelectedValue = 0
        lbePocketIndex9ComboBox.SelectedValue = 0
        lbePocketIndex10ComboBox.SelectedValue = 0
        lbePocketIndex11ComboBox.SelectedValue = 0
        lbePocketIndex12ComboBox.SelectedValue = 0
    End Sub

    Private Sub lbeClassComboBox_SelectedIndexChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles lbeClassComboBox.SelectedIndexChanged
        Select Case lbeClassComboBox.SelectedValue
            ' Nothing
            Case 0
                DisableAllPockets()
            Case 1
                EnableThighPackPockets()
            Case 2
                EnableVestPackPockets()
            Case 3
                EnableCombatPackPockets()
            Case 4
                EnableBackPackPockets()
        End Select
    End Sub
#End Region

#Region " Status Bar "
    Private Sub uiIndexUpDown_ValueChanged(ByVal sender As Object, ByVal e As System.EventArgs) Handles uiIndexUpDown.ValueChanged
        UpdateStatusBar()
    End Sub
#End Region


    Private Sub FoodTableLayout_Paint(sender As System.Object, e As System.Windows.Forms.PaintEventArgs)

    End Sub

    Public Enum ItemFlags
        Bloodbag = 0
        Manpad
        Beartrap
        Camera
        Waterdrum
        BloodcatMeat
        CowMeat
        Beltfed
        Ammobelt
        AmmobeltVest
        CamoRemoval
        Cleaningkit
        AttentionItem
        Garotte
        Covert
        Corpse
        BloodcatSkin
        NoMetalDetection
        JumpGrenade
        Handcuffs
        Taser
        ScubaBottle
        ScubaMask
        ScubaFins
        TripwireRoll
        Radioset
        SignalShell
        Soda
        RoofcollapseItem
        DiseaseprotectionFace
        DiseaseprotectionHand
        LBEexplosionproof
        EmptyBloodbag
        MedicalSplint
        Damageable
        Repairable
        WaterDamages
        Metal
        Sinks
        ShowStatus
        HiddenAddon
        TwoHanded
        NotBuyable
        Attachment
        HiddenAttachment
        BigGunList
        NotInEditor
        DefaultUndroppable
        Unaerodynamic
        Electronic
        Cannon
        RocketRifle
        FingerPrintID
        MetalDetector
        GasMask
        LockBomb
        Flare
        GrenadeLauncher
        Mortar
        Duckbill
        Detonator
        RemoteDetonator
        HideMuzzleFlash
        RocketLauncher
    End Enum

    Public Enum ItemFlags2
        SingleShotRocketLauncher = 0
        BrassKnuckles
        Crowbar
        GLGrenade
        FlakJacket
        LeatherJacket
        Batteries
        NeedsBatteries
        XRay
        WireCutters
        Toolkit
        FirstAidKit
        MedicalKit
        Canteen
        Jar
        CanAndString
        Marbles
        Walkman
        RemoteTrigger
        RobotRemoteControl
        CamouflageKit
        LocksmithKit
        Mine
        AntitankMine
        Hardware
        Medical
        GasCan
        ContainsLiquid
        Rock
        ThermalOptics
        SciFi
        NewInv
        DiseaseSystemExclusive
        Barrel
        TripWireActivation
        TripWire
        Directional
        BlockIronSight
        AllowClimbing
        Cigarette
        ProvidesRobotCamo
        ProvidesRobotNightVision
        ProvidesRobotLaserBonus
    End Enum

End Class

