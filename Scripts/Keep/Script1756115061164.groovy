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
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.testobject.ObjectRepository as OR

import com.kms.katalon.core.util.KeywordUtil

//import java.sql.*

String uniqueUserDataDir = "/tmp/chrome-profile-" + System.currentTimeMillis()

RunConfiguration.setWebDriverPreferencesProperty("args", [
	"--headless=new",
	"--user-data-dir=" + uniqueUserDataDir,
	"--disable-dev-shm-usage",
	"--no-sandbox",
	"--disable-gpu"
])

/*

// Connexion à PostgreSQL Docker
def url = "jdbc:postgresql://postgres-db:5432/testruns"
def user = "katalon"
def pass = "katalon123"

Connection conn = DriverManager.getConnection(url, user, pass)
Statement stmt = conn.createStatement()

// Création de la table si elle n'existe pas
stmt.execute("""
	CREATE TABLE IF NOT EXISTS test_results (
		id SERIAL PRIMARY KEY,
		execution_date TIMESTAMP,
		duration INTEGER,
		environment VARCHAR(50),
		ci_tool VARCHAR(50),
		test_suite_name VARCHAR(255),
		status VARCHAR(20),
		total_tests INTEGER,
		passed_tests INTEGER,
		failed_tests INTEGER,
		skipped_tests INTEGER,
		test_case_name VARCHAR(255),
		test_status VARCHAR(20),
		test_start_time TIMESTAMP,
		test_end_time TIMESTAMP,
		test_duration INTEGER,
		error_message TEXT,
		feature VARCHAR(100),
		test_type VARCHAR(50),
		last_success_date DATE,
		consecutive_failures INTEGER
	)
""")

conn.close()
println "✅ Table test_results vérifiée/créée avec succès."


*/

//GlobalVariable.loginUrl


//WebUI.openBrowser('')
WebUI.navigateToUrl('https://proxiserve.oc-sb.eu/auth/realms/opencell/protocol/openid-connect/auth?client_id=opencell-portal&redirect_uri=https%3A%2F%2Fproxiserve.oc-sb.eu%2Fopencell%2Ffrontend%2FDEMO%2Fportal%2F&state=3e59eaaa-a579-4236-b621-c86b50b05914&response_mode=fragment&response_type=code&scope=openid&nonce=edbd5617-6af7-4f0a-8f3c-4ed88fc6dc2d')

//WebUI.waitForPageLoad(5)

WebUI.setText(findTestObject('Object Repository/Test/Page_/input_Franais_username'), 'opencell.admin')
WebUI.setEncryptedText(findTestObject('Object Repository/Test/Page_/input_Franais_password'), 'w92AsKS6Pp5ImRK4eSzsTw==')
WebUI.click(findTestObject('Object Repository/Test/Page_/input_Franais_login'))

println "✅ Connexion envoyée avec succès."


//WebUI.openBrowser("https://proxiserve.oc-sb.eu/auth/realms/opencell/protocol/openid-connect/auth?client_id=opencell-portal&redirect_uri=https%3A%2F%2Fproxiserve.oc-sb.eu%2Fopencell%2Ffrontend%2FDEMO%2Fportal%2FB2B%2Finvoices%2Flist&state=9a28ca59-72f2-4ec0-89d7-16060e31e293&response_mode=fragment&response_type=code&scope=openid&nonce=85e2adda-7081-4de1-9f5e-882674b56151")
