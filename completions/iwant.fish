complete -c iwant -s C -l categories -d 'The specified categories [delimiter: ,] [default: all]' -r
complete -c iwant -s E -l exclude -d 'The excluded categories [delimiter: ,]' -r
complete -c iwant -s m -l extra-managers -d 'Enable extra managers [delimiter: ,]' -r -f -a "{paru	'',flatpak	'',npm	'',cargo	''}"
complete -c iwant -s s -l silent -d 'Don\'t display manifest'
complete -c iwant -s p -l preview -d 'View the manifest without downloading'
complete -c iwant -s h -l help -d 'Print help'
complete -c iwant -s V -l version -d 'Print version'
