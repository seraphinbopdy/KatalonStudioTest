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
    String ciTool = System.getenv("CI_TOOL") ?: "Jenkins"
    String gitBranch = System.getenv("GIT_BRANCH") ?: "main"
    String buildNumber = System.getenv("BUILD_NUMBER") ?: "local"
    String jobName = System.getenv("JOB_NAME") ?: "manual"

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

        println "📋 Test : ${testCaseContext.getTestCaseId()} [${status}] (${caseDuration}s)"

        try {
            def conn = getDBConnection()
            def stmt = conn.prepareStatement("""
                INSERT INTO test_results (
                    execution_date, environment, ci_tool, git_branch, job_name, build_number,
                    test_suite_name, test_case_name, test_status, test_start_time, test_end_time, 
                    test_duration, total_tests, passed_tests, failed_tests, skipped_tests,
                    katalon_version, browser_version
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """)

            stmt.setTimestamp(1, new Timestamp(suiteStartTime.getTime()))
            stmt.setString(2, environment)
            stmt.setString(3, ciTool)
            stmt.setString(4, gitBranch)
            stmt.setString(5, jobName)
            stmt.setString(6, buildNumber)

            stmt.setString(7, currentSuiteName)
            stmt.setString(8, testCaseContext.getTestCaseId())
            stmt.setString(9, status)
            stmt.setTimestamp(10, new Timestamp(caseStartTime.getTime()))
            stmt.setTimestamp(11, new Timestamp(caseEndTime.getTime()))
            stmt.setInt(12, (int)caseDuration)

            stmt.setInt(13, totalTests)
            stmt.setInt(14, passedTests)
            stmt.setInt(15, failedTests)
            stmt.setInt(16, skippedTests)

            stmt.setString(17, RunConfiguration.getVersion())
            stmt.setString(18, "Chrome " + com.kms.katalon.core.webui.driver.DriverFactory.getExecutedBrowser().getName())

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
}
