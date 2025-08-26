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


Map requestDataG = BuildDataGestionnaire.buildClientRequestData()
Map requestDataC = BuildDataCopropriete.buildCoproprietePayloadData()
Map requestDataCreateContrat = BuildDataCreateContrat.buildContratRequestData()
Map requestDataActiveContrat = BuildDataActivateContract.buildActivateContractData()

// Utilisation des données générées dans une requête API
def response1 = WS.sendRequest(findTestObject('API/CommunAPI/Gestionnaire', requestDataG))
assert response1.getStatusCode() == 200
def response2 = WS.sendRequest(findTestObject('API/CommunAPI/Copropriete', requestDataC))
assert response2.getStatusCode() == 200

println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataC))


def response3 = WS.sendRequest(findTestObject('API/TC001/CreateContratTC001', requestDataCreateContrat))
assert response3.getStatusCode() == 200
def response4 = WS.sendRequest(findTestObject('API/CommunAPI/ActivateContrat', requestDataActiveContrat))
assert response4.getStatusCode() == 200

WebUI.callTestCase(findTestCase('Test_Commun/TestCaseRTJOB'), [:], FailureHandling.STOP_ON_FAILURE)

println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataCreateContrat))
println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataC))

WebUI.callTestCase(findTestCase('TC001/GenerateFactureTC001'), [:], FailureHandling.STOP_ON_FAILURE)
WebUI.callTestCase(findTestCase('TC001/ValidationFactureTC001'), [:], FailureHandling.STOP_ON_FAILURE)


println response1.getResponseText()
println response2.getResponseText()

println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataC))
println JsonOutput.prettyPrint(JsonOutput.toJson(requestDataCreateContrat))




