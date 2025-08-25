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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://proxiserve.oc-sb.eu/auth/realms/opencell/protocol/openid-connect/auth?client_id=opencell-portal&redirect_uri=https%3A%2F%2Fproxiserve.oc-sb.eu%2Fopencell%2Ffrontend%2FDEMO%2Fportal%2FB2B%2Finvoices%2Flist&state=9a28ca59-72f2-4ec0-89d7-16060e31e293&response_mode=fragment&response_type=code&scope=openid&nonce=85e2adda-7081-4de1-9f5e-882674b56151')

WebUI.setText(findTestObject('Object Repository/Test/Page_/input_Franais_username'), 'opencell.admin')

WebUI.setEncryptedText(findTestObject('Object Repository/Test/Page_/input_Franais_password'), 'w92AsKS6Pp5ImRK4eSzsTw==')

WebUI.click(findTestObject('Object Repository/Test/Page_/input_Franais_kc-login'))

