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

import test_TC004_TC005_TC006_TC007.BuildDataGestionnaire
import test_TC004_TC005_TC006_TC007.BuildDataCopropriete
import test_TC004_TC005_TC006_TC007.BuildDataCreateContrat
import test_TC004_TC005_TC006_TC007.BuildDataActivateContract
import test_TC004_TC005_TC006_TC007.BuildDataDelaisReglementCopro


import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

// Créer un profil utilisateur Chrome unique
String uniqueUserDataDir = "/tmp/chrome-profile-" + System.currentTimeMillis()

// Ajouter les arguments Chrome
RunConfiguration.setWebDriverPreferencesProperty("args", [
	"--headless=new",
	"--user-data-dir=" + uniqueUserDataDir,
	"--disable-dev-shm-usage",
	"--no-sandbox",
	"--disable-gpu"
])

Map requestDataG = BuildDataGestionnaire.buildClientRequestData() // Nombre de jour par defaut pour le delais de règlement est 0 quand ce n'est pas précisé.
Map requestDataC = BuildDataCopropriete.buildCoproprietePayloadData()

Map requestDataCreateContrat = BuildDataCreateContrat.buildContratRequestData()
Map requestDataActiveContrat = BuildDataActivateContract.buildActivateContractData()
Map requestDataDelaisReglement = BuildDataDelaisReglementCopro.buildDelaisReglementData() // Nombre de jour par defaut pour le delais de règlement est 0 quand ce n'est pas précisé.



def response1 = WS.sendRequest(findTestObject('API/CommunAPI/Gestionnaire', requestDataG))
def response2 = WS.sendRequest(findTestObject('API/CommunAPI/Copropriete', requestDataC))

Thread.sleep(5000)
def responseUpdateCOPRO = WS.sendRequest(findTestObject('API/CommunAPI/DelaisReglement', requestDataDelaisReglement))

def response3 = WS.sendRequest(findTestObject('API/CommunAPI/CreateContrat', requestDataCreateContrat))
def response4 = WS.sendRequest(findTestObject('API/CommunAPI/ActivateContrat', requestDataActiveContrat))


println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataC))
println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataCreateContrat))


WebUI.callTestCase(findTestCase('Test_Commun/TestCaseRTJOB'), [:], FailureHandling.STOP_ON_FAILURE)
WebUI.callTestCase(findTestCase('Test_Commun/GenerateFacture'), [:], FailureHandling.STOP_ON_FAILURE)
WebUI.callTestCase(findTestCase('Test_Commun/ValidationFacture'), [:], FailureHandling.STOP_ON_FAILURE)



assert response1.getStatusCode() == 200
assert response2.getStatusCode() == 200
assert response3.getStatusCode() == 200
assert response4.getStatusCode() == 200

println response2.getResponseText()
println response1.getResponseText()
println response3.getResponseText() 
println response4.getResponseText()

println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataC))



