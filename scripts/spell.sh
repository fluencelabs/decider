if [[ -n "$2" ]];
then
    aqua run --addr "$LOCALADDR" --sk "$SK" -i scripts/spells.aqua -f "$1" --data-path "$2" --timeout 1000000
else
    aqua run --addr "$LOCALADDR" --sk "$SK" -i scripts/spells.aqua -f "$1" --timeout 1000000
fi
