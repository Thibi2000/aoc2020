for file in src/day*.rs; do
    file=${file#src/}
    DAYS=$DAYS:${file%.rs}';';
done
shopt -s extglob
MODS=$(echo $DAYS | sed  's/:/mod /g' | sed 's/;/;\\n/g')
SOLS=$(echo $DAYS | sed 's/:/solutions.push(/g' | sed 's/;/::solve);\\n/g')
sed --in-place "s|// <modules>|${MODS}|"  src/main.rs
sed --in-place "s|// <push>|${SOLS}|" src/main.rs
