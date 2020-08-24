# Contributing to HPermission

You're very welcomed to be a contributor to HPermission! 🎉

This document will show you the best practice of doing some contribution to this repo.

## Workflow

1. Fork HPermission on GitHub

   Visit [https://github.com/shuosc/HPermission](https://github.com/shuosc/HPermission)
   
   Click Fork button (top right) to establish a cloud-based fork.

2. Clone fork to local machine
   
   ```shell
   git clone git@github.com:$user/HPermission.git
   ```
   
   Set your clone to track upstream repository.
   
   ```shell
   git remote add upstream https://github.com/shuosc/HPermission
   # you definitely don't want to push directly to upstream
   git remote set-url --push upstream no_push
   ```

3. Branch

   We suggest you to create a branch for each feature you want to add or the bug you want to fix.
   
   Before that, get your local master up to date:
   
   ```shell
   # in the project dir
   git fetch upstream
   git checkout master
   git rebase upstream/master
   ```
   
   And then do the fork:
   
   ```shell
   git checkout -b myfeature
   ```

4. Develop

   Just do anything you want to the code!
   
5. Keep your branch in sync

   ```shell
   git fetch upstream
   git rebase upstream/master
   ```
   
6. Commit

   Before you commit, make sure that all the checks and unit tests are passed:
  
   ```shell
   make dev
   ```
   
   Then commit your changes.

   ```shell
   git commit
   ```
   
7. Push
   
   ```shell
   git push -f origin myfeature
   ```
   
8. Create a pull request

   Visit your fork at https://github.com/$user/HPermission (replace $user obviously).
   
   Click the Compare & pull request button next to your myfeature branch.
   
   Edit the description of the pull request to match your change.
   
   Your pull request needs to be reviewed and approved before get merged, 
   we matainers promise to check the pull requests now and then, 
   and you can ask for reviews from one of us if you can reach him ;)

## Don't know where to start from?

   We'll create some [good first issue](https://github.com/shuosc/HPermission/labels/good%20first%20issue) for first time contributors.
   
   We have mentors for every [good first issue](https://github.com/shuosc/HPermission/labels/good%20first%20issue), feel free ask for help from them.

## Need help? Have problems?

   Feel free to open an issue [here](https://github.com/shuosc/HPermission/issues)!
