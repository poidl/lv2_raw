LV2DIR=~/.lv2/
BACKUPDIR=~/programs/backup_lv2/
BUNDLE=eg-amp_rust.lv2
SONAME=libamp_rust.so
DEST=$LV2DIR$BUNDLE
if [ -d "$DEST" ]; then
    if [ -d "$BACKUPDIR/$BUNDLE" ]; then
        rm -r $BACKUPDIR/$BUNDLE
    fi
    mv $DEST $BACKUPDIR/$BUNDLE
fi
cp -r $BUNDLE $LV2DIR
cp target/debug/$SONAME $DEST
