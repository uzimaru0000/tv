_tv() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            tv)
                cmd="tv"
                ;;
            
            *)
                ;;
        esac
    done

    case "${cmd}" in
        tv)
            opts=" -h -V -r -s -a  --help --version --recursive --sort --align --style  <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --style)
                    COMPREPLY=($(compgen -W "ascii sharp rounded markdown plane" -- "${cur}"))
                    return 0
                    ;;

								--align)
										COMPREPLY=($(compgen -W "left center right" -- "${cur}"))
										return 0
										;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;

    esac
}

complete -F _tv -o bashdefault -o default tv