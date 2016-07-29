if [ $# -ne 1 ]; then
  echo 'Usage: subtree_diff NAME_OF_REPOSITORY'
  exit 1
fi
# why the "--"? see http://stackoverflow.com/questions/26349191/fatal-ambiguous-argument-branch-name-both-revision-and-filename
git diff-tree -p master:$1 $1 --