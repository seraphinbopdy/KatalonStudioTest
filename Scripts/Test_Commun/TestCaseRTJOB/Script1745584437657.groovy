import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import internal.GlobalVariable as GlobalVariable


TestObject requestObject = findTestObject('API/CommunAPI/RTJOB')

ResponseObject response = WS.sendRequest(requestObject)
println("Status Code du lancement du job: ${response.getStatusCode()}")

// Vérification de la réponse et traitement du JSON si le status est 200
if (response.getStatusCode() == 200) {
    def jsonSlurper = new JsonSlurper()
    def jsonResponse = jsonSlurper.parseText(response.getResponseBodyContent())

    // Vérification si l'ID du job est bien présent
    if (jsonResponse.actionStatus && jsonResponse.actionStatus.message) {
        int jobId = jsonResponse.actionStatus.message.toInteger()
        GlobalVariable.jobId = jobId  
        println("Job ID: ${jobId}") 

        // Préparation pour vérifier le statut du job avec une requête supplémentaire
        int maxAttempts = 20
        int delay = 3000 

        TestObject statusRequestObject = findTestObject('API/CommunAPI/CheckStatusJob')
        statusRequestObject.setFollowRedirects(true)

        // Boucle pour vérifier le statut du job
        for (int i = 0; i < maxAttempts; i++) {
            ResponseObject statusResponse = WS.sendRequest(statusRequestObject)

            if (statusResponse.getStatusCode() == 200) {
                def statusJson = jsonSlurper.parseText(statusResponse.getResponseBodyContent())

                if (statusJson.actionStatus && statusJson.actionStatus.status == "SUCCESS") {
                    println("Le job ${jobId} s'est terminé avec succès.")
                    break
                }
            } else {
                println("Tentative ${i + 1}: Échec de récupération du statut. Code HTTP: ${statusResponse.getStatusCode()}")
            }

            Thread.sleep(delay)
        }
    } else {
        println("Erreur : L'ID du job n'a pas été trouvé dans la réponse.")
    }
} else {
    println("Erreur : Échec du lancement du job. Status code: ${response.getStatusCode()}")
}
