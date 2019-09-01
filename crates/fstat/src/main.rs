use libc::{fstat, stat, S_IFMT, S_IFIFO, S_IFCHR, S_IFDIR, S_IFBLK, S_IFREG, S_IFLNK, S_IFSOCK, S_ISUID, S_ISGID, S_ISVTX, S_IRUSR, S_IWUSR, S_IXUSR};

fn main() {
    let stat = do_fstat(0);
    println!("is_stdin S_IFMT: {}", (stat.st_mode & S_IFMT) == S_IFMT);
    println!("is_stdin S_IFIFO: {}", (stat.st_mode & S_IFIFO) == S_IFIFO);
    println!("is_stdin S_IFCHR: {}", (stat.st_mode & S_IFCHR) == S_IFCHR);
    println!("is_stdin S_IFDIR: {}", (stat.st_mode & S_IFDIR) == S_IFDIR);
    println!("is_stdin S_IFBLK: {}", (stat.st_mode & S_IFBLK) == S_IFBLK);
    println!("is_stdin S_IFREG: {}", (stat.st_mode & S_IFREG) == S_IFREG);
    println!("is_stdin S_IFLNK: {}", (stat.st_mode & S_IFLNK) == S_IFLNK);
    println!("is_stdin S_IFSOCK: {}", (stat.st_mode & S_IFSOCK) == S_IFSOCK);
    println!("is_stdin S_ISUID: {}", (stat.st_mode & S_ISUID) == S_ISUID);
    println!("is_stdin S_ISGID: {}", (stat.st_mode & S_ISGID) == S_ISGID);
    println!("is_stdin S_ISVTX: {}", (stat.st_mode & S_ISVTX) == S_ISVTX);
    println!("is_stdin S_IRUSR: {}", (stat.st_mode & S_IRUSR) == S_IRUSR);
    println!("is_stdin S_IWUSR: {}", (stat.st_mode & S_IWUSR) == S_IWUSR);
    println!("is_stdin S_IXUSR: {}", (stat.st_mode & S_IXUSR) == S_IXUSR);
}

fn do_fstat (fd: i32) -> stat {
    let mut status: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atime_nsec: 0,
        st_mtime: 0,
        st_mtime_nsec: 0,
        st_ctime: 0,
        st_ctime_nsec: 0,
        st_birthtime: 0,
        st_birthtime_nsec: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0, 0],
    };
    unsafe {
        fstat(fd, &mut status);
    }
    status
}
