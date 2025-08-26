

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
import groovy.json.JsonSlurper
import groovy.json.JsonOutput
import java.text.SimpleDateFormat
import com.kms.katalon.core.testobject.ConditionType as ConditionType


import test_TC004_TC005_TC006_TC007.BuildDataGestionnaire
import test_TC004_TC005_TC006_TC007.BuildDataCopropriete
import test_TC004_TC005_TC006_TC007.BuildDataCreateContrat
import test_TC004_TC005_TC006_TC007.BuildDataActivateContract

Map requestDataG = BuildDataGestionnaire.buildClientRequestData([delaisReglement: 20])
Map requestDataC = BuildDataCopropriete.buildCoproprietePayloadData()
Map requestDataCreateContrat = BuildDataCreateContrat.buildContratRequestData()
Map requestDataActiveContrat = BuildDataActivateContract.buildActivateContractData()


TestObject testObject1 = new TestObject()
TestObject testObject2 = new TestObject()


String toFirstJanuary2025(Date inputDate) {
	Calendar calendar = Calendar.getInstance()
	calendar.setTime(inputDate)
	calendar.set(Calendar.YEAR, 2025)
	calendar.set(Calendar.MONTH, 0)
	calendar.set(Calendar.DAY_OF_MONTH, 1)
	SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd")
	return sdf.format(calendar.getTime())
}
Date today = new Date()


// Utilisation des données générées dans une requête API
def response1 = WS.sendRequest(findTestObject('API/CommunAPI/Gestionnaire', requestDataG))
assert response1.getStatusCode() == 200


GlobalVariable.codeCopro1 = GlobalVariable.COPRO_CodeCopropriete
requestDataC.codePropriétaire = GlobalVariable.codeCopro1


def response2 = WS.sendRequest(findTestObject('API/CommunAPI/Copropriete', requestDataC))
assert response2.getStatusCode() == 200


GlobalVariable.codeContrat1 = GlobalVariable.Contrat_CodeContrat
requestDataCreateContrat.codeContrat = GlobalVariable.codeContrat1
requestDataCreateContrat.dateDeLivraison = toFirstJanuary2025(today)
def response3 = WS.sendRequest(findTestObject('API/CommunAPI/CreateContrat', requestDataCreateContrat))


requestDataActiveContrat.codeCreationContrat = GlobalVariable.codeContrat1
def response4 = WS.sendRequest(findTestObject('API/CommunAPI/ActivateContrat', requestDataActiveContrat))
println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataC))
println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataCreateContrat)) 


/**
 * Get id de la copro et navigue via le lien contenant l'ID
 */


WebUI.callTestCase(findTestCase('Test_Commun/FindIdCopro'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Test_Commun/LoginPage'), [:], FailureHandling.STOP_ON_FAILURE)

def dynamicLink = "${GlobalVariable.BASE_URL_INVOICE}/opencell/frontend/DEMO/portal/B2B/billing-accounts/${GlobalVariable.identifiantCopro}/modify"

WebUI.callTestCase(findTestCase('Test_Commun/TestCaseRTJOB'), [:], FailureHandling.STOP_ON_FAILURE)
WebUI.navigateToUrl(dynamicLink)
Thread.sleep(3000)



testObject1.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='tab-tabs-clientCard-footer-TAB_CONSUMPTIONS']")
WebUI.click(testObject1)
Thread.sleep(3000)

testObject1.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='GENERATE_INVOICE']")
WebUI.click(testObject1)

testObject1.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='validateInvoiceBtn']")
WebUI.click(testObject1)

testObject1.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='invoice-actions-header']")
WebUI.click(testObject1)


Thread.sleep(3000)
testObject1.addProperty("xpath", ConditionType.EQUALS, "//div[@oc-testid='invoice-actions-header-CustomActionItem-GENERATE_PDF']")
WebUI.click(testObject1)


/**
 * Second testCase pour la Copro 2 ayant le meme gestionnaire.
 * Cree la Copro2 en utilisant le même code du gestionnaire
 */


Thread.sleep(5000)


Map requestDataC2 = BuildDataCopropriete.buildCoproprietePayloadData()
Map requestDataCreateContrat2 = BuildDataCreateContrat.buildContratRequestData()
Map requestDataActiveContrat2 = BuildDataActivateContract.buildActivateContractData()

def response7 = WS.sendRequest(findTestObject('API/CommunAPI/Copropriete', requestDataC2))
assert response7.getStatusCode() == 200

GlobalVariable.codeCopro2 = GlobalVariable.COPRO_CodeCopropriete
requestDataC2.codePropriétaire = GlobalVariable.codeCopro2

GlobalVariable.codeContrat2 = GlobalVariable.Contrat_CodeContrat
requestDataCreateContrat2.codeContrat = GlobalVariable.codeContrat2
requestDataCreateContrat2.dateDeLivraison = toFirstJanuary2025(today)
def response8 = WS.sendRequest(findTestObject('API/CommunAPI/CreateContrat', requestDataCreateContrat2))



requestDataActiveContrat2.codeCreationContrat = GlobalVariable.codeContrat2
def response9 = WS.sendRequest(findTestObject('API/CommunAPI/ActivateContrat', requestDataActiveContrat2))
println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataC2))
println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataCreateContrat2))




/**
 * Get id de la copro et navigue via le lien contenant l'ID
 */

WebUI.callTestCase(findTestCase('Test_Commun/FindIdCopro'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Test_Commun/LoginPage'), [:], FailureHandling.STOP_ON_FAILURE)

def dynamicLink2 = "${GlobalVariable.BASE_URL_INVOICE}/opencell/frontend/DEMO/portal/B2B/billing-accounts/${GlobalVariable.identifiantCopro}/modify"

WebUI.callTestCase(findTestCase('Test_Commun/TestCaseRTJOB'), [:], FailureHandling.STOP_ON_FAILURE)
WebUI.navigateToUrl(dynamicLink2)
Thread.sleep(3000)



testObject2.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='tab-tabs-clientCard-footer-TAB_CONSUMPTIONS']")
WebUI.click(testObject2)
Thread.sleep(3000)

testObject2.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='GENERATE_INVOICE']")
WebUI.click(testObject2)

testObject2.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='validateInvoiceBtn']")
WebUI.click(testObject2)

testObject2.addProperty("xpath", ConditionType.EQUALS, "//button[@oc-testid='invoice-actions-header']")
WebUI.click(testObject2)


Thread.sleep(3000)
testObject2.addProperty("xpath", ConditionType.EQUALS, "//div[@oc-testid='invoice-actions-header-CustomActionItem-GENERATE_PDF']")
WebUI.click(testObject2)
























































