# Continuous Integration or Headless Usage

A major use case for this toolkit is in continuous integration systems. This toolkit is intentionally simple and does not include any specific steps to allow developers to create their own tools. If you are interested in this you can use the following tools from the web:

* LabVIEW CLI Common Steps. Github: https://github.com/LabVIEW-DCAF/buildsystem/.
* Command Line Tools for LabVIEW. Github: https://github.com/chinghwayu/Command-Line-Tools

It has been successfully used with Jenkins and Gitlab CI and should support any CI tool that can call to the Windows command line.

## Powershell Usage

Powershell does not automatically detect non-zero error codes or standard error outputs as a failure and so some build systems may show the script as passing even after G-CLI reports an error. This is a specific design of powershell and is referred to as a non-terminating error.

For this reason try/catch blocks will not work with these either.

The best recommendation is to check the $? variable such as:

`if(!$?) { Exit $LASTEXITCODE }`

Some tools, such as Gitlab, insert these automatically after your commands (see https://docs.gitlab.com/runner/shells/)

## Avoiding LabVIEW Dialogs

LabVIEW dialogs will halt a headless system and make it appear to hang - resulting in a connection timeout error.

Since version 2.2 we set the unattended flag which should prevent dialogs from appearing and blocking the execution of our VIs.

However there are still two instances (at least) that I see.

### 1. Activation Dialog

I don't know there is anything that can be done for this.

### 2. Recovery Dialog

If LabVIEW crashes with unsaved changes the recovery dialog may show still. This has been reported as a bug.

You can disable this in the options of the LabVIEW install though. See https://knowledge.ni.com/KnowledgeArticleDetails?id=kA03q000000x1g6CAA&l=en-GB

## Configuring your build agent to show a UI

By default, build agents run as a service. However this means that they run without a user interface which can make it hard to debug.

The following instructions are used at Wiresmith Technology to make the UI visible. However they come with a significant side effect - the user needs to auto-login so anyone with access to the PC is logged in as that user and they can access the password of that user.

1. To limit the security exposure, create a user to run the build server.
2. Use [this tool](https://docs.microsoft.com/en-us/sysinternals/downloads/autologon) to configure the new user to log on automatically.
3. Download/Create your build agent. This process depends on the build server you are using but we want to end up with the agent executable and the command used to call it. Do not install it as a service.
4. Create a scheduled task in Windows to run the agent. Set it to run after login. Make sure to disable the timeout "stop the task if it runs longer than"

Now when a build runs you should be able to see LabVIEW running.