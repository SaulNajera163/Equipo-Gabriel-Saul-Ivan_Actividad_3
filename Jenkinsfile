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
                    sh 'git --version || echo "Git is not installed"'
                    // Verificar Docker
                    sh 'docker --version || echo "Docker is not installed"'
                    // Verificar que la imagen existe
                    sh 'docker images ${DOCKER_IMAGE}:${DOCKER_TAG} || echo "Docker image not found"'
                }
            }
        }

        stage('Prueba de API') {
            steps {
                script {
                    try {
                        // Intentar detener el contenedor si existe
                        sh 'docker stop ${CONTAINER_NAME} || true'
                        sh 'docker rm ${CONTAINER_NAME} || true'
                        
                        // Iniciar el contenedor
                        sh 'docker run -d --name ${CONTAINER_NAME} -p ${APP_PORT}:${APP_PORT} ${DOCKER_IMAGE}:${DOCKER_TAG}'
                        
                        // Esperar a que la API esté lista
                        sh 'sleep 10'
                        
                        // Prueba básica
                        sh 'curl -f http://localhost:${APP_PORT} || echo "API test failed"'
                    } catch (Exception e) {
                        echo "Error durante las pruebas: ${e.message}"
                        currentBuild.result = 'FAILURE'
                    }
                }
            }
        }
    }

    post {
        always {
            // Limpiar
            sh 'docker stop ${CONTAINER_NAME} || true'
            sh 'docker rm ${CONTAINER_NAME} || true'
        }
    }
}