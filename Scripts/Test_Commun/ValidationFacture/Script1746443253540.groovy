import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.util.KeywordUtil;
import org.junit.Assert;

import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import groovy.json.JsonSlurper
import groovy.json.JsonOutput

import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent

import internal.GlobalVariable as GlobalVariable


import test_TC004_TC005_TC006_TC007.BuildDataValidateInvoice


// Fonction pour convertir l'ID de la facture en String et le nettoyer
String getCleanedFactureId(def idFacture) {
	if (idFacture && !idFacture.isEmpty()) {
		// Convertir le premier élément de la liste en String et retirer les crochets s'il y en a
		return idFacture.get(0).toString().replaceAll("\\[", "").replaceAll("\\]", "")
	} else {
		// Retourner une chaîne vide ou un résultat par défaut si la liste est vide
		return ""
	}
}


// Fonction pour nettoyer l'ID de la facture et le convertir en long
long getCleanedFactureIdAsLong(def idFacture) {
	if (idFacture && !idFacture.isEmpty()) {
		String factureIdStr = idFacture.get(0).toString().replaceAll("\\[", "").replaceAll("\\]", "")
		return Long.parseLong(factureIdStr)
	} else {
		throw new IllegalArgumentException("L'ID de la facture est invalide ou manquant.")
	}
}


WebUI.openBrowser("${GlobalVariable.BASE_URL_INVOICE}/auth/realms/opencell/protocol/openid-connect/auth?client_id=opencell-portal&redirect_uri=https%3A%2F%2Fspa2.oc-sb.eu%2Fopencell%2Ffrontend%2FDEMO%2Fportal%2F&state=113e95c2-0a90-4ccb-98e2-cef9ac0def21&response_mode=fragment&response_type=code&scope=openid&nonce=283de7a0-ca80-4d4c-9e60-6b2085a0ed27&code_challenge=8jnaM_hoFIes7nFTbrw2E2YBtXZ2UHhc2-lrjKq1As8&code_challenge_method=S256")

TestObject dynamicObject = new TestObject()

TestObject clickCustomer = new TestObject()


WebUI.setText(findTestObject('LoginPage/input_Franais_username'), 'opencell.admin')

WebUI.setEncryptedText(findTestObject('LoginPage/input_Franais_password'), 'w92AsKS6Pp5ImRK4eSzsTw==')

WebUI.click(findTestObject('LoginPage/input_Franais_login'))


clickCustomer.addProperty("xpath", ConditionType.EQUALS, "//*[@oc-testid= 'MenuItem-B2B']")
WebUI.click(clickCustomer)
 
 
Map requestDataValidation = BuildDataValidateInvoice.buildActivateInvoiceData()


TestObject testObject = findTestObject('API/CommunAPI/ValidationFacture',requestDataValidation)



def idFacture = getCleanedFactureId(GlobalVariable.idFactureObtenu)
// Construction du lien dynamique avec l'ID de la Facture et navigue directement vers le lien
String dynamicLink = "${GlobalVariable.BASE_URL_INVOICE}/opencell/frontend/DEMO/portal/B2B/invoices/${idFacture}/modify"

Thread.sleep(5000)
WebUI.navigateToUrl(dynamicLink)
println("Il se dirige bien vers la page de la facture: ${dynamicLink}")

/**
 * Validation de la facture à travers le button de  Validation de la Facture.
 */
def response = WS.sendRequest(testObject)
println("Code de statut: " + response.getStatusCode())
println("Réponse: " + response.getResponseText())
println("l'ID de la Facture pour la validation est: ${GlobalVariable.idFactureObtenu}")

 
 
 