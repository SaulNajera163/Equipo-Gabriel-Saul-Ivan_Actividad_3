pipeline {
    agent any

    environment {
        DOCKER_IMAGE = 'api_estudiantes'
        DOCKER_TAG = 'latest'
        CONTAINER_NAME = 'api_test_container'
        APP_PORT = '8000'
    }

    stages {
        stage('Verificación de Ambiente') {
            steps {
                script {
                    // Verificar Git
                    sh 'git --version'
                    // Verificar Docker
                    sh 'docker --version'
                    // Verificar que la imagen existe
                    sh 'docker images ${DOCKER_IMAGE}:${DOCKER_TAG}'
                }
            }
        }
    }
}