# So far this is only used for $1="lv2". Lv2 is developed within yassy,
# but is kept in a separate repo.
# See https://git-scm.com/book/en/v2/Git-Tools-Advanced-Merging#_subtree_merge

if [ $# -ne 1 ]; then
  echo 'Usage: subtree_branch.sh NAME_OF_REPOSITORY'
  exit 1
fi

git remote add $1 git@github.com:poidl/$1.git
git fetch $1
git checkout -b $1 $1/master

# # make script in branch to merge changes on master
# mkdir scripts
# echo "git merge --squash -s subtree master" >> scripts/merge_master.sh
# chmod u+x scripts/merge_master.sh
# git add .
# git commit -m "add script to merge changes on master"

# switch back to master and copy repo into subfolder
git checkout master
# is this necessary?
git merge -s ours --no-commit  $1/master
git read-tree --prefix=$1/ -u $1/master
git commit -m "Subtree merged in "$1