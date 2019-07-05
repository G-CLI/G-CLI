<?xml version='1.0' encoding='UTF-8'?>
<Project Type="Project" LVVersion="11008008">
	<Item Name="My Computer" Type="My Computer">
		<Property Name="server.app.propertiesEnabled" Type="Bool">true</Property>
		<Property Name="server.control.propertiesEnabled" Type="Bool">true</Property>
		<Property Name="server.tcp.enabled" Type="Bool">false</Property>
		<Property Name="server.tcp.port" Type="Int">0</Property>
		<Property Name="server.tcp.serviceName" Type="Str">My Computer/VI Server</Property>
		<Property Name="server.tcp.serviceName.default" Type="Str">My Computer/VI Server</Property>
		<Property Name="server.vi.callsEnabled" Type="Bool">true</Property>
		<Property Name="server.vi.propertiesEnabled" Type="Bool">true</Property>
		<Property Name="specify.custom.address" Type="Bool">false</Property>
		<Item Name="Echo CWD.vi" Type="VI" URL="../Echo CWD.vi"/>
		<Item Name="Echo Parameters.vi" Type="VI" URL="../Echo Parameters.vi"/>
		<Item Name="Generate Large Output.vi" Type="VI" URL="../Generate Large Output.vi"/>
		<Item Name="Quit With Parameter Code.vi" Type="VI" URL="../Quit With Parameter Code.vi"/>
		<Item Name="Dependencies" Type="Dependencies">
			<Item Name="vi.lib" Type="Folder">
				<Item Name="BuildHelpPath.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/BuildHelpPath.vi"/>
				<Item Name="Check Special Tags.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Check Special Tags.vi"/>
				<Item Name="Clear Errors.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Clear Errors.vi"/>
				<Item Name="CLI.lvclass" Type="LVClass" URL="/&lt;vilib&gt;/Wiresmith Technology/G CLI/CLI Class/CLI.lvclass"/>
				<Item Name="Convert property node font to graphics font.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Convert property node font to graphics font.vi"/>
				<Item Name="Details Display Dialog.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Details Display Dialog.vi"/>
				<Item Name="DialogType.ctl" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/DialogType.ctl"/>
				<Item Name="DialogTypeEnum.ctl" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/DialogTypeEnum.ctl"/>
				<Item Name="Error Cluster From Error Code.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Error Cluster From Error Code.vi"/>
				<Item Name="Error Code Database.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Error Code Database.vi"/>
				<Item Name="ErrWarn.ctl" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/ErrWarn.ctl"/>
				<Item Name="eventvkey.ctl" Type="VI" URL="/&lt;vilib&gt;/event_ctls.llb/eventvkey.ctl"/>
				<Item Name="Find Tag.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Find Tag.vi"/>
				<Item Name="Format Message String.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Format Message String.vi"/>
				<Item Name="General Error Handler CORE.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/General Error Handler CORE.vi"/>
				<Item Name="General Error Handler.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/General Error Handler.vi"/>
				<Item Name="Get String Text Bounds.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Get String Text Bounds.vi"/>
				<Item Name="Get Text Rect.vi" Type="VI" URL="/&lt;vilib&gt;/picture/picture.llb/Get Text Rect.vi"/>
				<Item Name="GetHelpDir.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/GetHelpDir.vi"/>
				<Item Name="GetRTHostConnectedProp.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/GetRTHostConnectedProp.vi"/>
				<Item Name="Longest Line Length in Pixels.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Longest Line Length in Pixels.vi"/>
				<Item Name="LVBoundsTypeDef.ctl" Type="VI" URL="/&lt;vilib&gt;/Utility/miscctls.llb/LVBoundsTypeDef.ctl"/>
				<Item Name="Not Found Dialog.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Not Found Dialog.vi"/>
				<Item Name="Search and Replace Pattern.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Search and Replace Pattern.vi"/>
				<Item Name="Set Bold Text.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Set Bold Text.vi"/>
				<Item Name="Set String Value.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Set String Value.vi"/>
				<Item Name="Simple Error Handler.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Simple Error Handler.vi"/>
				<Item Name="TagReturnType.ctl" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/TagReturnType.ctl"/>
				<Item Name="Three Button Dialog CORE.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Three Button Dialog CORE.vi"/>
				<Item Name="Three Button Dialog.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Three Button Dialog.vi"/>
				<Item Name="Trim Whitespace.vi" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/Trim Whitespace.vi"/>
				<Item Name="whitespace.ctl" Type="VI" URL="/&lt;vilib&gt;/Utility/error.llb/whitespace.ctl"/>
			</Item>
		</Item>
		<Item Name="Build Specifications" Type="Build">
			<Item Name="CWD" Type="EXE">
				<Property Name="AB_Class_Path" Type="Path">/C/Program Files (x86)/National Instruments/LabVIEW 2011/vi.lib/AppBuilder/AB_Classes/EXE/AB_EXE.lvclass</Property>
				<Property Name="AB_UIClass_Path" Type="Path">/C/Program Files (x86)/National Instruments/LabVIEW 2011/vi.lib/AppBuilder/AB_Classes/Build/UI/AB_UI_Frmwk_Build.lvclass</Property>
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{2D8DBAA2-E6A5-4F52-9BC6-5A7629A6953F}</Property>
				<Property Name="App_INI_GUID" Type="Str">{33E21490-37A7-468A-B5B9-4B2859467FF3}</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{1F630A7D-D94D-4CDA-A648-10EC5CA54687}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">CWD</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Integration Tests/exes</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{15371F17-DE20-4506-B1EC-37201A5CF17F}</Property>
				<Property Name="Destination[0].destName" Type="Str">Echo CWD.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Integration Tests/exes/Echo CWD.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Integration Tests/exes/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{B964C433-7256-4C15-961A-7722673825CE}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Echo Parameters.vi</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="Source[2].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[2].itemID" Type="Ref">/My Computer/Echo CWD.vi</Property>
				<Property Name="Source[2].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[2].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">3</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">Echo</Property>
				<Property Name="TgtF_fileVersion.major" Type="Int">1</Property>
				<Property Name="TgtF_internalName" Type="Str">Echo</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2018 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">Echo</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{25AD2EBD-4913-43BE-B4F8-C961F3918EEE}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">Echo CWD.exe</Property>
			</Item>
			<Item Name="Echo" Type="EXE">
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{A1E337D9-0713-4F04-938F-1784BB7BC680}</Property>
				<Property Name="App_INI_GUID" Type="Str">{273DE972-4F0C-496F-82CB-3638121A576E}</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{E13EDC37-6572-4014-B9C3-6ABDE9ABC702}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">Echo</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Integration Tests/exes</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{C8E42A72-D7DC-49F9-B0E4-C0EAFB8C235E}</Property>
				<Property Name="Destination[0].destName" Type="Str">Echo CLI.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Integration Tests/exes/Echo CLI.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Integration Tests/exes/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{9A2659A8-0278-44C2-B0A9-434E315938A8}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Echo Parameters.vi</Property>
				<Property Name="Source[1].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">2</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">Echo</Property>
				<Property Name="TgtF_fileVersion.major" Type="Int">1</Property>
				<Property Name="TgtF_internalName" Type="Str">Echo</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2018 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">Echo</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{707811E1-37F5-4DE3-AC1C-71DC14E8B7F3}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">Echo CLI.exe</Property>
			</Item>
			<Item Name="Generate Large Output" Type="EXE">
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{5001801B-9302-4DC3-BD58-0378CD7D97AA}</Property>
				<Property Name="App_INI_GUID" Type="Str">{600DA4A3-A4E5-4AC7-B205-C1613FBA840A}</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{56D521FF-2B6E-4441-AE28-6D6AD9E9052D}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">Generate Large Output</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Integration Tests/exes</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{075581F0-1246-4D1E-9F20-B4E9D1192F72}</Property>
				<Property Name="Destination[0].destName" Type="Str">LargeOutput.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Integration Tests/exes/LargeOutput.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Integration Tests/exes/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{9A2659A8-0278-44C2-B0A9-434E315938A8}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Generate Large Output.vi</Property>
				<Property Name="Source[1].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">2</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">Generate Large Output</Property>
				<Property Name="TgtF_fileVersion.major" Type="Int">1</Property>
				<Property Name="TgtF_internalName" Type="Str">Generate Large Output</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2018 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">Generate Large Output</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{D4889F51-6F25-445E-9614-EAB0B3EACC73}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">LargeOutput.exe</Property>
			</Item>
			<Item Name="Quit With Code" Type="EXE">
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{EBF392C0-CCBA-4042-9B1E-0F9429EA6024}</Property>
				<Property Name="App_INI_GUID" Type="Str">{0D555E4D-F99A-47D2-B770-F93D5FCF7B49}</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{CCF70B65-02A7-4216-9BF1-4CB86465A315}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">Quit With Code</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Integration Tests/exes</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{A8482325-6E8F-4436-8AB4-2DA76A77AB59}</Property>
				<Property Name="Destination[0].destName" Type="Str">QuitWithCode.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Integration Tests/exes/QuitWithCode.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Integration Tests/exes/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{9A2659A8-0278-44C2-B0A9-434E315938A8}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Quit With Parameter Code.vi</Property>
				<Property Name="Source[1].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">2</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">Quit With Code</Property>
				<Property Name="TgtF_fileVersion.major" Type="Int">1</Property>
				<Property Name="TgtF_internalName" Type="Str">Quit With Code</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2018 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">Quit With Code</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{95F68BBD-6A18-4784-B6DD-317D06F295A8}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">QuitWithCode.exe</Property>
			</Item>
		</Item>
	</Item>
</Project>
