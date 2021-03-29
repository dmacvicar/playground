FROM gradle:jdk11 as build
COPY --chown=gradle:gradle . /home/gradle/src
WORKDIR /home/gradle/src
RUN gradle bootJar

FROM opensuse/leap:15.2
RUN zypper install -y java-11-openjdk-headless
RUN mkdir -p /app
WORKDIR /app
COPY --from=build /home/gradle/src/build/libs/hello-0.0.1-SNAPSHOT.jar .
ENTRYPOINT ["java", "-jar", "/app/hello-0.0.1-SNAPSHOT.jar"]
