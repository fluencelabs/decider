for spell in $(./scripts/spell.sh 'list()' | jq '.[]');
do
	./scripts/spell.sh "remove($spell)"
done
