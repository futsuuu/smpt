$env.PROMPT_INDICATOR = ""
$env.PROMPT_INDICATOR_VI_INSERT = ""
$env.PROMPT_INDICATOR_VI_NORMAL = ""
$env.PROMPT_MULTILINE_INDICATOR = "    "
$env.PROMPT_COMMAND = { || ^::SMPT:: run $env.LAST_EXIT_CODE  }
$env.PROMPT_COMMAND_RIGHT = ""
