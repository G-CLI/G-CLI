pipeline {
agent none

	stages {
		stage("VS Building") {
			agent {label 'VS14'}
			stages {
				stage ('Get VS Dependencies') {	
					steps {
						bat 'nuget restore \"C Sharp Source/LabVIEW CLI/LabVIEW CLI.sln\"'
					}
				}

				stage ('VS Build') {
					steps {
						bat "\"${tool 'MS Build'}\" \"C Sharp Source/LabVIEW CLI/LabVIEW CLI.sln\" /p:Configuration=Release /p:Platform=\"Any CPU\""
						bat "\"${tool 'MS Build'}\" \"C Sharp Source/LabVIEW CLI/LabVIEW CLI.sln\" /p:Configuration=Release /p:Platform=\"x64\""
						bat "MoveInstallers.bat"
						stash name:"installers", includes:"LabVIEW Source/Installation Support/*.msi"
					}
				}
					
				stage ('VS Test'){
					steps{
						bat "if exist VSTestResults.trx del VSTestResults.trx"
						bat "\"C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\Common7\\IDE\\mstest.exe\" /resultsfile:\"%WORKSPACE%/VSTestResults.trx\" /testcontainer:\"%WORKSPACE%/C Sharp Source/LabVIEWCLI_Unit_tests/bin/Release/LabVIEWCLI_Unit_tests.dll\" /nologo"
						step([$class: 'MSTestPublisher', testResultsFile:"VSTestResults.trx", failOnError: true, keepLongStdio: true])
					}
				}
				
					
				stage ('VS Integration Test') {
					steps {
						bat 'pushd \"Integration Tests\" & \"Run Integration Tests.bat\" \"../C Sharp Source/LabVIEW CLI/bin/x64/Release/" & popd'
					}
				}
			}
		}
			
		stage ('LabVIEW Builds') {
			agent {label 'LV2011'}
			stages {
				stage ('Get Dependencies') {
					steps {
						unstash 'installers'
					}
				}
				
				stage ('LabVIEW Build') {
					steps {
						bat "labview-cli -v \"C:\\Users\\Public\\Documents\\National Instruments\\LV-CLI Common Steps\\steps\\vipbBuild.vi\" -- \"LabVIEW Source\\LabVIEW CLI.vipb\" Builds  \"${env.WORKSPACE}\""
						archiveArtifacts artifacts:'Builds/*.vipb'
					}
				}
			
			}
		}
		

	}
}