git remote add lv2 git@github.com:poidl/lv2.git
git fetch lv2
git checkout -b lv2 lv2/master
git checkout master
git merge -s ours --no-commit --allow-unrelated-histories lv2_remote/master
git merge -s ours --no-commit  lv2_remote/master
git branch
git read-tree --prefix=lv2/ -u lv2_remote/master
ls lv2/
git commit -m "Subtree merged in lv2"