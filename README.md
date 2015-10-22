# SafeNetworkTestUploader
Quick and dirty test uploader/downloader for safe network

Command line tool to upload small files for testing into safe network.

Usage:
(upload): uploadutil upl <local file> <remote folder>
(download): uploadutil dl <remote file> <local path>
(mkdir): uploadutil mkdir <remote path>

Login details are supplied in three line text file which contains keyword pin and password.
File location should be in TEST_SAFENETWORK_LOGIN_PATH enviroment variable.

Example:

-------
test
1234
test
-------

