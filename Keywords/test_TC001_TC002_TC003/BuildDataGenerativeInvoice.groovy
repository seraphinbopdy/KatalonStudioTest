package test_TC001_TC002_TC003

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable
import java.text.SimpleDateFormat
import java.util.Calendar


public class BuildDataGenerativeInvoice {

	@Keyword
	static Map buildGenerateInvoiceData(Map addMap = [:]) {
		SimpleDateFormat formatter = new SimpleDateFormat("yyyy-MM-dd")


		// 1er jour du mois courant
		Calendar firstOfMonth = Calendar.getInstance()
		firstOfMonth.set(Calendar.DAY_OF_MONTH, 1)
		String setInvoiceDateFirstOfMonth = formatter.format(firstOfMonth.getTime())

		// 20 du mois suivant
		Calendar twentiethNextMonth = Calendar.getInstance()
		twentiethNextMonth.add(Calendar.MONTH, 1)
		twentiethNextMonth.set(Calendar.DAY_OF_MONTH, 20)
		String setAtTwentyOfMonthPlusOne = formatter.format(twentiethNextMonth.getTime())

		def data = [
			codeCoprio: GlobalVariable.COPRO_CodeCopropriete,
			setInvoiceDateFirstOfMonth: setInvoiceDateFirstOfMonth,
			setAtTwentyOfMonthPlusOne : setAtTwentyOfMonthPlusOne
		]

		Map finalData = data + addMap
		return finalData
	}
}
