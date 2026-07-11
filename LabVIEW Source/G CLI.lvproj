<?xml version='1.0' encoding='UTF-8'?>
<Project Type="Project" LVVersion="17008000">
	<Property Name="CCSymbols" Type="Str">ENABLE_ALL_TESTS,False;</Property>
	<Property Name="NI.LV.All.SaveVersion" Type="Str">17.0</Property>
	<Property Name="NI.LV.All.SourceOnly" Type="Bool">true</Property>
	<Property Name="NI.Project.Description" Type="Str"></Property>
	<Item Name="My Computer" Type="My Computer">
		<Property Name="CCSymbols" Type="Str"></Property>
		<Property Name="IOScan.Faults" Type="Str"></Property>
		<Property Name="IOScan.NetVarPeriod" Type="UInt">100</Property>
		<Property Name="IOScan.NetWatchdogEnabled" Type="Bool">false</Property>
		<Property Name="IOScan.Period" Type="UInt">10000</Property>
		<Property Name="IOScan.PowerupMode" Type="UInt">0</Property>
		<Property Name="IOScan.Priority" Type="UInt">9</Property>
		<Property Name="IOScan.ReportModeConflict" Type="Bool">true</Property>
		<Property Name="IOScan.StartEngineOnDeploy" Type="Bool">false</Property>
		<Property Name="NI.SortType" Type="Int">3</Property>
		<Property Name="server.app.propertiesEnabled" Type="Bool">true</Property>
		<Property Name="server.control.propertiesEnabled" Type="Bool">true</Property>
		<Property Name="server.tcp.enabled" Type="Bool">false</Property>
		<Property Name="server.tcp.port" Type="Int">0</Property>
		<Property Name="server.tcp.serviceName" Type="Str">My Computer/VI Server</Property>
		<Property Name="server.tcp.serviceName.default" Type="Str">My Computer/VI Server</Property>
		<Property Name="server.vi.callsEnabled" Type="Bool">true</Property>
		<Property Name="server.vi.propertiesEnabled" Type="Bool">true</Property>
		<Property Name="specify.custom.address" Type="Bool">false</Property>
		<Item Name="Help" Type="Folder">
			<Item Name="G CLI Getting Started.vi" Type="VI" URL="../Help/G CLI Getting Started.vi"/>
		</Item>
		<Item Name="Tools" Type="Folder">
			<Item Name="Echo.vi" Type="VI" URL="../Tools/Echo.vi"/>
			<Item Name="QuitLabVIEW.vi" Type="VI" URL="../Tools/QuitLabVIEW.vi"/>
			<Item Name="ClearCache.vi" Type="VI" URL="../Tools/ClearCache.vi"/>
		</Item>
		<Item Name="Tests" Type="Folder">
			<Item Name="Options Parsing Tests.lvclass" Type="LVClass" URL="../Tests/Options Parsing Tests/Options Parsing Tests.lvclass"/>
			<Item Name="Working Path Expansion Tests.lvclass" Type="LVClass" URL="../Tests/Working Path Expansion Tests/Working Path Expansion Tests.lvclass"/>
			<Item Name="Service Name Tests.lvclass" Type="LVClass" URL="../Tests/Service Name Tests/Service Name Tests.lvclass"/>
		</Item>
		<Item Name="Examples" Type="Folder">
			<Item Name="CLI Demo.vi" Type="VI" URL="../CLI Demo.vi"/>
			<Item Name="Options Parser Example.vi" Type="VI" URL="../Examples/Options Parser Example.vi"/>
		</Item>
		<Item Name="Option Parser.lvlib" Type="Library" URL="../Argument Parser/Option Parser.lvlib"/>
		<Item Name="CLI.lvclass" Type="LVClass" URL="../CLI Class/CLI.lvclass"/>
		<Item Name="Integration Tests.lvlib" Type="Library" URL="../integration-tests/Integration Tests.lvlib"/>
		<Item Name="TestLib.lvlib" Type="Library" URL="../integration-tests/TestLib.lvlib"/>
		<Item Name="Events.ctl" Type="VI" URL="../CLI Class/Events.ctl"/>
		<Item Name="Dependencies" Type="Dependencies"/>
		<Item Name="Build Specifications" Type="Build">
			<Item Name="CWD Test" Type="EXE">
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{57FF0EAF-1EB4-4A65-B57F-A2F2191A90E3}</Property>
				<Property Name="App_INI_GUID" Type="Str">{74187064-D5A6-4185-B15B-352321946A06}</Property>
				<Property Name="App_serverConfig.httpPort" Type="Int">8002</Property>
				<Property Name="Bld_autoIncrement" Type="Bool">true</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{729DE10F-5A3A-4FFB-B1DE-C4068F8C82A5}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">CWD Test</Property>
				<Property Name="Bld_excludeInlineSubVIs" Type="Bool">true</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Builds</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{0E74C042-9798-4311-86EF-20E5E3D06DCE}</Property>
				<Property Name="Bld_version.build" Type="Int">3</Property>
				<Property Name="Bld_version.major" Type="Int">1</Property>
				<Property Name="Destination[0].destName" Type="Str">Echo CWD.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Builds/Echo CWD.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Builds/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{EC913425-D37F-40DC-A3C3-101CFE798C4F}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Echo CWD.vi</Property>
				<Property Name="Source[1].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">2</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">CWD Test</Property>
				<Property Name="TgtF_internalName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2023 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{E5F19AEE-A7F7-4FE0-A32D-CA49609BDD73}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">Echo CWD.exe</Property>
			</Item>
			<Item Name="Echo Test" Type="EXE">
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{FE23823C-E894-430B-B072-DCDCC4F09834}</Property>
				<Property Name="App_INI_GUID" Type="Str">{ED9B5A93-3EB3-4A5B-9455-2A3571993E9D}</Property>
				<Property Name="App_serverConfig.httpPort" Type="Int">8002</Property>
				<Property Name="Bld_autoIncrement" Type="Bool">true</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{B3EFC76F-1EAE-4D9E-8E03-BC6E1F63AA84}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">Echo Test</Property>
				<Property Name="Bld_excludeInlineSubVIs" Type="Bool">true</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Builds</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{93131568-F627-49AD-BDDF-D47D4102A4DE}</Property>
				<Property Name="Bld_version.build" Type="Int">3</Property>
				<Property Name="Bld_version.major" Type="Int">1</Property>
				<Property Name="Destination[0].destName" Type="Str">Echo CLI.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Builds/Echo CLI.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Builds/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{EC913425-D37F-40DC-A3C3-101CFE798C4F}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Echo CWD.vi</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="Source[2].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[2].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Echo Parameters.vi</Property>
				<Property Name="Source[2].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[2].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">3</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">CWD Test</Property>
				<Property Name="TgtF_internalName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2023 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{0912C70E-368B-4BE0-AF51-1446A99FBD77}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">Echo CLI.exe</Property>
			</Item>
			<Item Name="Large Output Test" Type="EXE">
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{457E7FC1-5C07-4E0E-9434-B277F01A847F}</Property>
				<Property Name="App_INI_GUID" Type="Str">{4D073099-2DE9-41A9-ACD4-380C81DF56AC}</Property>
				<Property Name="App_serverConfig.httpPort" Type="Int">8002</Property>
				<Property Name="Bld_autoIncrement" Type="Bool">true</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{0FB44694-5472-47A6-B750-83083CA82989}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">Large Output Test</Property>
				<Property Name="Bld_excludeInlineSubVIs" Type="Bool">true</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Builds</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{C65C192F-A1FD-42E0-A71D-AD0664B8BC7C}</Property>
				<Property Name="Bld_version.build" Type="Int">3</Property>
				<Property Name="Bld_version.major" Type="Int">1</Property>
				<Property Name="Destination[0].destName" Type="Str">LargeOutput.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Builds/LargeOutput.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Builds/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{EC913425-D37F-40DC-A3C3-101CFE798C4F}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Echo CWD.vi</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="Source[2].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[2].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Echo Parameters.vi</Property>
				<Property Name="Source[2].type" Type="Str">VI</Property>
				<Property Name="Source[3].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[3].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Generate Large Output.vi</Property>
				<Property Name="Source[3].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[3].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">4</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">CWD Test</Property>
				<Property Name="TgtF_internalName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2023 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{96E01E61-E8AF-4F87-8F34-A78C026974FF}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">LargeOutput.exe</Property>
			</Item>
			<Item Name="Quit with Code Test" Type="EXE">
				<Property Name="App_copyErrors" Type="Bool">true</Property>
				<Property Name="App_INI_aliasGUID" Type="Str">{A8CD0028-A6D3-4C38-B656-31A1F9E37760}</Property>
				<Property Name="App_INI_GUID" Type="Str">{FCE0C5FE-DF8D-4083-8B1F-B11CC5DCB7CC}</Property>
				<Property Name="App_serverConfig.httpPort" Type="Int">8002</Property>
				<Property Name="Bld_autoIncrement" Type="Bool">true</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{7E0A6F08-3728-4B63-ADF7-78729176CFB7}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">Quit with Code Test</Property>
				<Property Name="Bld_excludeInlineSubVIs" Type="Bool">true</Property>
				<Property Name="Bld_excludeLibraryItems" Type="Bool">true</Property>
				<Property Name="Bld_excludePolymorphicVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Builds</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_modifyLibraryFile" Type="Bool">true</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{8439AB50-81E3-4A3A-8FEE-A623A2B2525E}</Property>
				<Property Name="Bld_version.build" Type="Int">3</Property>
				<Property Name="Bld_version.major" Type="Int">1</Property>
				<Property Name="Destination[0].destName" Type="Str">QuitWithCode.exe</Property>
				<Property Name="Destination[0].path" Type="Path">../Builds/QuitWithCode.exe</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Builds/data</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="Source[0].itemID" Type="Str">{EC913425-D37F-40DC-A3C3-101CFE798C4F}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Echo CWD.vi</Property>
				<Property Name="Source[1].type" Type="Str">VI</Property>
				<Property Name="Source[2].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[2].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Echo Parameters.vi</Property>
				<Property Name="Source[2].type" Type="Str">VI</Property>
				<Property Name="Source[3].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[3].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Generate Large Output.vi</Property>
				<Property Name="Source[3].type" Type="Str">VI</Property>
				<Property Name="Source[4].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[4].itemID" Type="Ref">/My Computer/Integration Tests.lvlib/Quit With Parameter Code.vi</Property>
				<Property Name="Source[4].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[4].type" Type="Str">VI</Property>
				<Property Name="SourceCount" Type="Int">5</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">CWD Test</Property>
				<Property Name="TgtF_internalName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2023 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">CWD Test</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{4043C307-A7ED-40E2-965D-DFDEFD5EE83F}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">QuitWithCode.exe</Property>
			</Item>
			<Item Name="Test In Packed Library" Type="Packed Library">
				<Property Name="Bld_autoIncrement" Type="Bool">true</Property>
				<Property Name="Bld_buildCacheID" Type="Str">{8983A3B4-EFDE-4F72-BB91-AD62754455E7}</Property>
				<Property Name="Bld_buildSpecName" Type="Str">Test In Packed Library</Property>
				<Property Name="Bld_excludeInlineSubVIs" Type="Bool">true</Property>
				<Property Name="Bld_localDestDir" Type="Path">../Builds</Property>
				<Property Name="Bld_localDestDirType" Type="Str">relativeToCommon</Property>
				<Property Name="Bld_previewCacheID" Type="Str">{BE5AAD23-2852-4E10-8342-A83569163CB8}</Property>
				<Property Name="Bld_version.build" Type="Int">6</Property>
				<Property Name="Bld_version.major" Type="Int">1</Property>
				<Property Name="Destination[0].destName" Type="Str">Tests.lvlibp</Property>
				<Property Name="Destination[0].path" Type="Path">../Builds/Tests.lvlibp</Property>
				<Property Name="Destination[0].preserveHierarchy" Type="Bool">true</Property>
				<Property Name="Destination[0].type" Type="Str">App</Property>
				<Property Name="Destination[1].destName" Type="Str">Support Directory</Property>
				<Property Name="Destination[1].path" Type="Path">../Builds</Property>
				<Property Name="DestinationCount" Type="Int">2</Property>
				<Property Name="PackedLib_callersAdapt" Type="Bool">true</Property>
				<Property Name="Source[0].itemID" Type="Str">{EC913425-D37F-40DC-A3C3-101CFE798C4F}</Property>
				<Property Name="Source[0].type" Type="Str">Container</Property>
				<Property Name="Source[1].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[1].itemID" Type="Ref">/My Computer/Integration Tests.lvlib</Property>
				<Property Name="Source[1].Library.allowMissingMembers" Type="Bool">true</Property>
				<Property Name="Source[1].type" Type="Str">Library</Property>
				<Property Name="Source[2].destinationIndex" Type="Int">0</Property>
				<Property Name="Source[2].itemID" Type="Ref">/My Computer/TestLib.lvlib</Property>
				<Property Name="Source[2].Library.allowMissingMembers" Type="Bool">true</Property>
				<Property Name="Source[2].Library.atomicCopy" Type="Bool">true</Property>
				<Property Name="Source[2].Library.LVLIBPtopLevel" Type="Bool">true</Property>
				<Property Name="Source[2].preventRename" Type="Bool">true</Property>
				<Property Name="Source[2].sourceInclusion" Type="Str">TopLevel</Property>
				<Property Name="Source[2].type" Type="Str">Library</Property>
				<Property Name="SourceCount" Type="Int">3</Property>
				<Property Name="TgtF_companyName" Type="Str">Wiresmith Technology</Property>
				<Property Name="TgtF_fileDescription" Type="Str">Test In Packed Library</Property>
				<Property Name="TgtF_internalName" Type="Str">Test In Packed Library</Property>
				<Property Name="TgtF_legalCopyright" Type="Str">Copyright © 2023 Wiresmith Technology</Property>
				<Property Name="TgtF_productName" Type="Str">Test In Packed Library</Property>
				<Property Name="TgtF_targetfileGUID" Type="Str">{87871011-AB53-4FB3-82C9-7127321A9A5B}</Property>
				<Property Name="TgtF_targetfileName" Type="Str">Tests.lvlibp</Property>
			</Item>
		</Item>
	</Item>
</Project>
