node {
    
stage 'Checkout'
    checkout([$class: 'GitSCM', branches: [[name: '*/master']], doGenerateSubmoduleConfigurations: false, extensions: [], submoduleCfg: [], userRemoteConfigs: [[credentialsId: '971d4d17-6a91-4d53-8e1d-66051cd122d5', url: 'ssh://git@github.com/JamesMc86/LabVIEW-CLI.git']]])

stage 'VS Build'
    bat 'nuget restore \"C Sharp Source/LabVIEW CLI/LabVIEW CLI.sln\"'
    bat "\"${tool 'MS Build'}\" \"C Sharp Source/LabVIEW CLI/LabVIEW CLI.sln\" /p:Configuration=Release /p:Platform=\"Any CPU\""
	
stage 'VS Test'
    bat "if exist VSTestResults.trx del VSTestResults.trx"
	bat "\"C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\Common7\\IDE\\mstest.exe\" /resultsfile:\"%WORKSPACE%/VSTestResults.trx\" /testcontainer:\"%WORKSPACE%/C Sharp Source/LabVIEWCLI_Unit_tests/bin/Release/LabVIEWCLI_Unit_tests.dll\" /nologo"
	step([$class: 'MSTestPublisher', testResultsFile:"VSTestResults.trx", failOnError: true, keepLongStdio: true])
    
stage 'Integration Test'
    bat 'pushd \"Integration Tests\" & \"Run Integration Tests.bat\" \"../C Sharp Source/LabVIEW CLI/bin/Release/" & popd'

}