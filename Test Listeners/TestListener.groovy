import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile

import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext

import com.kms.katalon.core.annotation.*
import com.kms.katalon.core.configuration.RunConfiguration

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.driver.DriverFactory

import java.sql.*
import java.util.Date

class TestListener {
	
		def suiteStartTime
		def caseStartTime
		def caseEndTime
	
		def totalTests = 0
		def passedTests = 0
		def failedTests = 0
		def skippedTests = 0
	
		def currentSuiteName = "unknown"
	
		String environment = System.getenv("ENV") ?: "QA"
		String ciTool      = System.getenv("CI_TOOL") ?: "Jenkins"
		String gitBranch   = System.getenv("GIT_BRANCH") ?: "main"
		String buildNumber = System.getenv("BUILD_NUMBER") ?: "local"
		String jobName     = System.getenv("JOB_NAME") ?: "manual"
		String hostname    = InetAddress.localHost.hostName
	
		@BeforeTestSuite
		def beforeTestSuite(TestSuiteContext testSuiteContext) {
			suiteStartTime = new Date()
			currentSuiteName = testSuiteContext.getTestSuiteId()
			println "🟢 Début suite : ${currentSuiteName}"
		}
	
		@BeforeTestCase
		def beforeTestCase(TestCaseContext testCaseContext) {
			caseStartTime = new Date()
		}
	
		@AfterTestCase
		def afterTestCase(TestCaseContext testCaseContext) {
			caseEndTime = new Date()
			long caseDuration = (caseEndTime.time - caseStartTime.time) / 1000
	
			totalTests++
			def status = testCaseContext.getTestCaseStatus()
			if (status == "PASSED") passedTests++
			else if (status == "FAILED") failedTests++
			else skippedTests++
	
			// Détection type de test
			String testType = detectTestType(testCaseContext)
	
			// Feature = le dossier du test case (ex: "TC001" → "Login")
			String feature = extractFeature(testCaseContext.getTestCaseId())
	
			println "📋 Test : ${testCaseContext.getTestCaseId()} [${status}] (${caseDuration}s) - Type=${testType}"
	
			try {
				def conn = getDBConnection()
				def stmt = conn.prepareStatement("""
                INSERT INTO test_results (
                    execution_date, environment, ci_tool, git_branch, job_name, build_number, hostname,
                    test_suite_name, test_case_name, test_status, test_start_time, test_end_time, 
                    test_duration, total_tests, passed_tests, failed_tests, skipped_tests,
                    katalon_version, browser, feature, test_type, error_message
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """)
	
				stmt.setTimestamp(1, new Timestamp(suiteStartTime.getTime()))
				stmt.setString(2, environment)
				stmt.setString(3, ciTool)
				stmt.setString(4, gitBranch)
				stmt.setString(5, jobName)
				stmt.setString(6, buildNumber)
				stmt.setString(7, hostname)
	
				stmt.setString(8, currentSuiteName)
				stmt.setString(9, testCaseContext.getTestCaseId())
				stmt.setString(10, status)
				stmt.setTimestamp(11, new Timestamp(caseStartTime.getTime()))
				stmt.setTimestamp(12, new Timestamp(caseEndTime.getTime()))
				stmt.setInt(13, (int)caseDuration)
	
				stmt.setInt(14, totalTests)
				stmt.setInt(15, passedTests)
				stmt.setInt(16, failedTests)
				stmt.setInt(17, skippedTests)
	
				stmt.setString(18, RunConfiguration.getAppVersion())
				stmt.setString(19, getBrowserInfo())
				stmt.setString(20, feature)
				stmt.setString(21, testType)
				stmt.setString(22, status == "FAILED" ? testCaseContext.getMessage() : null)
	
				stmt.executeUpdate()
				conn.close()
	
				println "✅ Résultat inséré en DB"
	
			} catch (Exception e) {
				println "❌ Erreur DB : ${e.message}"
			}
		}
	
		private static Connection getDBConnection() {
			String url = "jdbc:postgresql://postgres-db:5432/testruns"
			String user = "katalon"
			String pass = "katalon123"
			Class.forName("org.postgresql.Driver")
			return DriverManager.getConnection(url, user, pass)
		}
	
		private static String detectTestType(TestCaseContext ctx) {
			try {
				def content = ctx.getTestCaseVariables()?.toString() ?: ""
				if (content.contains("WS.")) return "API"
				if (content.contains("WebUI.")) return "UI"
				if (content.contains("Mobile.")) return "MOBILE"
			} catch (Exception ignored) {}
			return "UNKNOWN"
		}
	
		private static String extractFeature(String testCaseId) {
			if (testCaseId.contains("/")) {
				return testCaseId.split("/")[1]  // Exemple: "Test Cases/TC001/Implementation" → "TC001"
			}
			return "Generic"
		}
	
		private static String getBrowserInfo() {
			try {
				def browser = DriverFactory.getExecutedBrowser()
				return browser != null ? browser.getName() : "N/A"
			} catch (Exception e) {
				return "N/A"
			}
		}
	}