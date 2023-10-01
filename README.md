# rex - Rename Exchange

uses renameat2(2) to swap content

command line interface to atomically swap content of two files.

## Why

When dual-booting between Linux and a Windows partitions I sometimes experience
drives not wanting to mount according to `/etc/fstab`. This tends to happen when
mounting a ntfs drive that is mounted both when booting from Windows partition
and by Linux partition. This leads me to edit `/etc/fstab` removing the drives
that fail to mount such that I can deal with that issue later. This leads to me
maintaining two variants of `/etc/fstab`. One is the ideal where every drive is
mounted and the other is the safe variant where only the non-ntfs drives are
mounted.

Now every time I want to switch between these two variants I have to at least
three commands (of a combination of `mv` and `cp` or just three `mv`).

`rex` is for this use-case. It swaps file variants atomically.

```sh
# switch
rex /etc/fstab /etc/ftstab.variant
# switch back
rex /etc/fstab /etc/ftstab.variant
```

### Current workflow

```sh
cd config
cp important.config important.config.prev
# work on the alternative config
vim important.config
# test important program

# we tested and are now ready to use the previous config again
# and we save the edited important.config for later

cp important.config important.config.new
mv important.config.prev important.config
rm important.config.prev

# It is now later and we want to continue working on important.config.new

cp important.config important.config.prev
mv important.config.new important.config
rm important.config.new

# work on the alternative config
vim important.config


# this is the workflow
```

### Alternative Workflow

```sh
cd config
cp important.config important.config.variant

# Start work on the new variant
vim important.config
# test important program

# we tested and are now ready to use the original variant again
# and we would like to save the edited important.config variant for later

rex important.config important.config.variant

# It is now later and we want to continue working on important.config.new

rex important.config important.config.variant

vim important.config

# this is the workflow
```