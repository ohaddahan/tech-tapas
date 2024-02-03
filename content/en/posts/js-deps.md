+++
title = 'Tips for safer dependency management in JavaScript environments'
date = 2024-01-28T09:22:33+02:00
draft = false
+++

![Image description](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/lg64youncuy41gksf37m.jpeg)

Just about anyone who worked on the JavaScript ecosystem has faced annoying issues due to version mismatch ,
dependencies updates etc. , I want to share a few small tips that helped me mitigate some of the issues.

## Tip #1 [save-exact](https://docs.npmjs.com/cli/v9/using-npm/config#save-exact)

`npm install <package-name> --save-exact`

This will install the package exact version instead of default range.

`"dependencies": {
"axios": "1.6.5"
}`

Due note that `package-lock.json` will still contain ranges. One way to mitigate this is to manually edit the file, a
simple `regex` will enforce exact versions on the dependencies too.

(if some dependencies requirements can't be fulfilled with strict nested versions, you can iterate until you find a set
of matching versions and make them static)

For example running the code below in `vim` editor.

`%s/: "\^/: "/g`

You can set it as the default behavior by running.

`npm config set save-exact=true`

Which will set `save-exact=true` in your `.npmrc`.

## Tip #2 avoid named releases

One way to help synchronize multiple developers working on the same repository is using `nvm` (or similar) with the
appropriate `rc` file such as `.nvmrc`.

`nvm` supports named versions which might look nicer on a first look but generates various issues, since these versions
aren't static.

Causing issues with compatibility with various dependencies , and other random issues such as running `nvm use` but it
no longer finds the version, since it was updated and you need to reinstall it locally.

It's even more problematic once you take into account `CI/CD` since they reinstall the environment from scratch each
time and it will probably differ from local developers environment that haven't yet update to the latests named version
update.

Hence I highly recommend using a version number and simply change it as needed, knowingly.

## Tip #3 use [engines](https://docs.npmjs.com/cli/v10/configuring-npm/package-json#engines)

Basically an expansion on #2 , explicitly name the `node` / `npm` version (strict, not a range!) to avoid more
unexpected and unwanted surprises.

## Tip #4 use [engine-strict](https://docs.npmjs.com/cli/v9/using-npm/config#engine-strict)

Avoid installing potential packages that might cause issues.

It's better to resolve an issue early on, with little overhead at the start than fixing it later when it's a critical
part of your system.

