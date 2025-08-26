import groovy.json.JsonSlurper
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.testobject.RequestObject


import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import java.text.SimpleDateFormat
import java.util.Date
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonOutput
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.util.KeywordUtil
import org.junit.Assert

import org.openqa.selenium.Keys as Keys

import test_TC004_TC005_TC006_TC007.BuildDataGenerativeInvoice

Map requestDataAGenerateInvoice = BuildDataGenerativeInvoice.buildGenerateInvoiceData()

TestObject testObject = ObjectRepository.findTestObject('API/CommunAPI/GenerateFacture',requestDataAGenerateInvoice)
ResponseObject response = WS.sendRequest(testObject)

try {
	Assert.assertEquals(200, response.getStatusCode())
	println("Le statut HTTP est 200. Traitement du contenu de la réponse...")
	
	// Parse le contenu JSON de la réponse
	def jsonSlurper = new JsonSlurper()
	def jsonResponse = jsonSlurper.parseText(response.getResponseBodyContent())

	// Extraction de l'ID de facture à partir de la réponse JSON
	def invoiceId = jsonResponse.id
	GlobalVariable.idFactureObtenu = invoiceId
	if (invoiceId == null || invoiceId.toString().trim().isEmpty()) {
		println("⚠️ Erreur : L'ID de la facture est vide ou null.")
	} else {
		// Assurer que c'est bien une chaîne de caractères propre
		invoiceId = invoiceId.toString().replaceAll("[\\[\\]]", "")
		println("ID de la facture extrait : " + invoiceId)

		// Construction d'un lien dynamique avec l'ID de facture
		String dynamicLink = "https://proxiserve.oc-sb.eu/opencell/frontend/DEMO/portal/B2B/invoices/${invoiceId}/modify"
		println("Lien dynamique pour modification de la facture : " + dynamicLink)
	}
} catch (AssertionError e) {
	KeywordUtil.markError("Erreur d'assertion : Statut HTTP attendu était 200, mais obtenu " + response.getStatusCode())
}










