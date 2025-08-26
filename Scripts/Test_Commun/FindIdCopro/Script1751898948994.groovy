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
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import groovy.json.JsonSlurper





def variable = [
	
	'findIdCoproByCodeCorpo': GlobalVariable.COPRO_CodeCopropriete
	
	]
	
	
TestObject testObject = ObjectRepository.findTestObject('API/CommunAPI/FindIdCoproByCodeCopro',variable)
def response = WS.sendRequest(testObject)

def jsonSlurper = new JsonSlurper()
def jsonResponse = jsonSlurper.parseText(response.getResponseBodyContent())

def jobIdCopro = jsonResponse.data.id[0] as Integer

GlobalVariable.identifiantCopro = jobIdCopro

def url = "${GlobalVariable.BASE_URL_INVOICE}/opencell/frontend/DEMO/portal/B2B/billing-accounts/${GlobalVariable.identifiantCopro}/modify"

System.err.println("Affiche l'Id de la reponse: " + jobIdCopro)
System.err.println("Affiche le Lien a acceder: " + url)



