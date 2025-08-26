package test_TC004_TC005_TC006_TC007

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
import com.github.javafaker.Faker

import java.text.SimpleDateFormat
import java.util.Calendar

public class BuildDataCreateContrat {

	@Keyword
	static Map buildContratRequestData(Map override = [:]) {

		Faker faker = new Faker()


		String codeContratRandom = "25CO0000" + faker.number().numberBetween(100, 999)
		GlobalVariable.Contrat_CodeContrat = codeContratRandom


		/*
		 String fixedDate = new SimpleDateFormat("yyyy-MM-dd").format(
		 new SimpleDateFormat("dd/MM/yyyy").parse("01/06/2025")
		 )
		 */

		//Obtenir la date du premier jour du mois courant
		Calendar calendar = Calendar.getInstance()
		calendar.set(Calendar.DAY_OF_MONTH, 1)
		String fixedDate = new SimpleDateFormat("yyyy-MM-dd").format(calendar.getTime())


		Map localData = [
			codeContrat      : codeContratRandom,
			dateSouscription : fixedDate,
			codeCopropriete  : GlobalVariable.COPRO_CodeCopropriete,
			dateDeLivraison  : fixedDate
		]

		return localData + override
	}
}



