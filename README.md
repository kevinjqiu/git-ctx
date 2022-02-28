```
          _ __             __      
   ____ _(_) /_      _____/ /__  __
  / __ `/ / __/_____/ ___/ __/ |/_/
 / /_/ / / /_/_____/ /__/ /__>  <  
 \__, /_/\__/      \___/\__/_/|_|  
/____/                             
```

Easily switch between recently used git branches

Usage
=====

Drop the executable in any directory on your `$PATH`. Then:

    git ctx

## List recent branches:

    git ctx list-branches  # or git ctx l

e.g.,

    > git ctx l
    [*] new-branch
        master
        old-branch-1
        old-branch-2

## Switch branches:

    git ctx switch-branch  # or git ctx s

e.g.,

    > git ctx s
    [0] -->new-branch<--
    [1] master
    [2] old-branch-1
    [3] old-branch-2
    ---------------------
    Enter the branch number you want to switch to:

    2
    Your branch is up to date with 'origin/old-branch-1'.

    Previous HEAD position was ...
    Switched to branch 'old-branch-1'
