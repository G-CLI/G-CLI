# Options Parser Library

There is a library included for parsing options from the CLI. Note - there is also a new library which I have yet to evaluate and compare at https://www.vipm.io/package/sklein_lib_lv_argparse/ which may suit your needs better.
The options parser enables you to create more complex interfaces with optional parameters to make it easier to use in simple cases.


For example if we have an interface like:

` -v -lv-ver 2014 myproject mytarget `

We would say that `-v` and `-lv-ver` are option labels and `2014` is the value for the `-lv-ver` option label. `myproject` and `mytarget` are trailing arguments.

# API
## Create Option Set
![](images/create%20option%20set.png)

This VI initialises the options set ready to parse what is sent to your application. You need to specify the options you support through the input array. In each cluster you can set:

* Option type - a flag means there is no associated value e.g. -v for verbose. A value means there is a value to parse afterwards e.g. -lv-ver 2014.
* Option Labels - An array of the labels that are associated with the option on the command line. For example the verbose option might respond to "v" or "verbose".
* Key - The name you will use to access this option once parsed.
* Description - A help text description for the option. Right now this is unused but useful for generating help responses (which we hope to support in the future).
* Required - If this is set to required, the parser will flag if this is missing.
* Default Value - The default used if this isn't set.

## Parse Arguments
![](images/parse%20arguments.png)

Pass this the options set and the arguments from the command line in order to parse out the different options you used above.

You can access the options later through the option set object. This will flag if required options are missing at this point.

You can change the label marker input to change how the options are detected. This is a regex input so you can get creative but for ease of use I would recommend sticking to the default `-`. 

## Get Option Value
![](images/get%20option.png)

Use this to access the value for a given option after you have parsed the input.

The key is the key you specified in when configuring the option set. You will get the string value back for that option (which will be the default if un-set) and you can detect if the user actually specified this option (the *set* output).

## Get Trailing Arguments
![](images/get%20trailing%20arguments.png)

This returns an array of all arguments that weren't associated with an option flag during parsing. Commonly these are the key required elements of your API.