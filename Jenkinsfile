pipeline {
    agent any

    stages {
        // Etapa 1: Prueba de inicio
        stage('Test') {
            steps {
                echo 'Prueba de pipeline básico'
            }
        }

        // Etapa 2: Validar entorno (opcional)
        stage('Check Environment') {
            steps {
                echo "Verificando entorno de Jenkins"
                sh 'java -version || echo "Java no está instalado"'
                sh 'docker --version || echo "Docker no está instalado"'
            }
        }

        // Etapa 3: Ejecutar un contenedor Docker de prueba
        stage('Run Docker Test') {
            steps {
                echo "Levantando un contenedor de prueba"
                sh '''
                docker run --rm hello-world
                '''
            }
        }

        // Etapa 4: Clonar repositorio (prueba Git)
        stage('Clone Repository') {
            steps {
                echo 'Clonando el repositorio desde GitHub'
                git url: 'https://github.com/SaulNajera163/Equipo-Gabriel-Saul-Ivan_Actividad_3.git', branch: 'main'
            }
        }
    }
}
