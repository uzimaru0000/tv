complete -c tv -l style -d 'Table style' -r -f -a "ascii sharp rounded markdown plane"
complete -c tv -s a -l align -d 'Table alignment' -r -f -a "left center right"
complete -c tv -s s -l sort -d 'Options for sorting by key' -r -f
complete -c tv -l no-headers -d 'Specify that the input has no header row'
complete -c tv -s r -l recursive -d 'Recursive display'
complete -c tv -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c tv -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
