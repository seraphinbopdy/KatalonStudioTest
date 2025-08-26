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



WebUI.openBrowser("${GlobalVariable.BASE_URL_INVOICE}/auth/realms/opencell/protocol/openid-connect/auth?client_id=opencell-portal&redirect_uri=https%3A%2F%2Fspa2.oc-sb.eu%2Fopencell%2Ffrontend%2FDEMO%2Fportal%2F&state=113e95c2-0a90-4ccb-98e2-cef9ac0def21&response_mode=fragment&response_type=code&scope=openid&nonce=283de7a0-ca80-4d4c-9e60-6b2085a0ed27&code_challenge=8jnaM_hoFIes7nFTbrw2E2YBtXZ2UHhc2-lrjKq1As8&code_challenge_method=S256")

TestObject dynamicObject = new TestObject()

TestObject clickCustomer = new TestObject()


WebUI.setText(findTestObject('LoginPage/input_Franais_username'), 'opencell.admin')

WebUI.setEncryptedText(findTestObject('LoginPage/input_Franais_password'), 'w92AsKS6Pp5ImRK4eSzsTw==')

WebUI.click(findTestObject('LoginPage/input_Franais_login'))

clickCustomer.addProperty("xpath", ConditionType.EQUALS, "//*[@oc-testid='MenuItem-B2B']")

WebUI.waitForElementClickable(clickCustomer, 10)

// Scroll au cas où l’élément est masqué
WebUI.scrollToElement(clickCustomer, 5)

WebUI.click(clickCustomer)
