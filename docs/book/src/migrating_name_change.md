For version 2.0 we change the library name from LabVIEW CLI to G CLI to avoid confusion with NI's tool that they released.

In changing the name there are a couple of points at which things may break. This document outlines those and I ask you report any additional problems you may find.

# VI Package

Since the VI package name has changed VIPM will not recognised the upgrade. You will need to uninstall the old package and install the new package manually.

# Code Linking

The folder containing the code has changed from `<vi.lib>/Wiresmith Technology/LabVIEW CLI` to `<vi.lib>/Wiresmith Technology/G CLI`. Since all functions are contained within the `CLI.lvclass` you should only need to relink to this. I would advise mass compiling your code which calls to the CLI to fix the links.

# Command Line Command

The new tool is now called `g-cli` to be called from the command line. However it will continue to work with `labview-cli`. For new scripts though I recommend moving to `g-cli` in case we have to remove the labview-cli link in the future.

You may need to restart any CI tools or other callers so they recognise the change in the path variables.