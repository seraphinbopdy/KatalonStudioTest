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
import com.github.javafaker.Faker

import internal.GlobalVariable


public class BuildDataGestionnaire {


	@Keyword
	static Map buildClientRequestData(Map addMap = [:]) {

		Faker faker = new Faker()

		String codeGestionnaire = "GEST_" + faker.number().numberBetween(10000, 99999).toString()
		GlobalVariable.GEST_codeGestionnaire = codeGestionnaire

		Map localData = [
			descriptionName : faker.company().name(),
			email           : faker.internet().emailAddress(),
			infoEmail       : faker.internet().emailAddress(),
			zipCode         : faker.number().digits(5),
			address1        : faker.address().streetAddress(),
			address2        : faker.address().streetName(),
			address3        : faker.address().secondaryAddress(),
			delaisReglement : faker.number().numberBetween(15, 60),
			GEST_codeGestionnaire: codeGestionnaire,
			delaisReglement: addMap.containsKey('delaisReglement') ? addMap.delaisReglement : ''
		]

		if (addMap.containsKey('customerCategory')) {
			localData.customerCategory = addMap.customerCategory
		}

		// Merge avec addMap (si tu veux forcer des valeurs spécifiques)
		Map finalData = localData + addMap

		return finalData
	}
}


