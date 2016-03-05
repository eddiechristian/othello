# othello
###To run eclipse with rustDT
Numbered list:
1.)applications/Docker Terminal  ---first you need to start a docker terminal(special terminal)
2.) docker images     ---next list built/downloaded images
3.)create environment variable file:
	JAVA_HOME=/opt/jdk1.8.0_73/
	PATH="/root/.cargo/bin:/opt/eclipse/:$JAVA_HOME/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
4.)socat TCP-LISTEN:6000,reuseaddr,fork UNIX-CLIENT:\"$DISPLAY\" ---this may not be required.

4.) next run eddiechristian/rust :0.1  docker image(eclipse w/ rustDT)
    docker run --rm -it   --env-file <env file>  -v <hostdir>:<contianer mount>  eddiechristian/rust:0.1    /bin/bash
5.) run eclipse

