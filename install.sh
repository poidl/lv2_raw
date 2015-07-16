LV2DIR=~/.lv2/
BACKUPDIR=~/programs/backup_lv2/
BUNDLE=brrrsynth.lv2
SONAME=libbrrsynth.so
DEST=$LV2DIR$BUNDLE
if [ -d "$DEST" ]; then
    if [ -d "$BACKUPDIR/$BUNDLE" ]; then
        rm -r $BACKUPDIR/$BUNDLE
    fi
    mv $DEST $BACKUPDIR/$BUNDLE
fi
cp -r $BUNDLE $LV2DIR
cp target/debug/$SONAME $DEST
