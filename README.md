# othello
###Steps to run rust Docker Image 
1.)**applications/Docker Terminal**  ---first you need to start a docker terminal(special terminal)  
2.)**list Docker images** built/downloaded images  

```bash
	$docker images  
```
3.)**create environment variable file** 
```bash 
	JAVA_HOME=/opt/jdk1.8.0_73/  
	PATH="/root/.cargo/bin:/opt/eclipse/:$JAVA_HOME/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"  
	RUST_SRC_PATH=/opt/rustc-1.6.0/src/  
```
4.)**start socat** (this may not be required)  
```bash
    socat TCP-LISTEN:6000,reuseaddr,fork UNIX-CLIENT:\"$DISPLAY\"  
```
4.)**run  docker image**(eclipse w/ rustDT)  
```bash
    docker run --rm -it --env-file ./env.list -v /Users/eddie/rust/othello:/othello eddiechristian/rust:0.1 /bin/bash
```
5.**run eclipse**  

