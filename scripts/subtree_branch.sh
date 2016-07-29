if [ $# -ne 1 ]; then
  echo 'Usage: subtree_branch.sh NAME_OF_REPOSITORY'
  exit 1
fi

git remote add $1 git@github.com:poidl/$1.git
git fetch $1
git checkout -b $1 $1/master
git checkout master
#git merge -s ours --no-commit  $1/master
git read-tree --prefix=$1/ -u $1/master
git commit -m "Subtree merged in "$1