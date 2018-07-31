node {
    
stage 'Checkout'
    checkout scm
	
stage 'Get Dependencies'
    bat 'nuget restore \"C Sharp Source/LabVIEW CLI/LabVIEW CLI.sln\"'

stage 'VS Build'
    bat "\"${tool 'MS Build'}\" \"C Sharp Source/LabVIEW CLI/LabVIEW CLI.sln\" /p:Configuration=Release /p:Platform=\"Any CPU\""
	
stage 'VS Test'
    bat "if exist VSTestResults.trx del VSTestResults.trx"
	bat "\"C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\Common7\\IDE\\mstest.exe\" /resultsfile:\"%WORKSPACE%/VSTestResults.trx\" /testcontainer:\"%WORKSPACE%/C Sharp Source/LabVIEWCLI_Unit_tests/bin/Release/LabVIEWCLI_Unit_tests.dll\" /nologo"
	step([$class: 'MSTestPublisher', testResultsFile:"VSTestResults.trx", failOnError: true, keepLongStdio: true])
    
stage 'Integration Test'
    bat 'pushd \"Integration Tests\" & \"Run Integration Tests.bat\" \"../C Sharp Source/LabVIEW CLI/bin/Release/" & popd'

}