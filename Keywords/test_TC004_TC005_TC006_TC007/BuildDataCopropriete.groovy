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

public class BuildDataCopropriete {

	@Keyword
	static Map buildCoproprietePayloadData(Map addMap = [:]) {

		Faker faker = new Faker()

		def codeCopropriete = "COPRO_" + faker.number().numberBetween(10000, 99999)
		GlobalVariable.COPRO_CodeCopropriete = codeCopropriete


		def codeGestionnaire = GlobalVariable.GEST_codeGestionnaire

		Map localData = [
			'codePropriétaire'       : GlobalVariable.COPRO_CodeCopropriete,
			'descriptionName'        : faker.company().name(),
			'zipCode'                : faker.number().digits(5),
			'email'                  : faker.internet().emailAddress(),
			'GEST_codeGestionnaire'  : codeGestionnaire,
			'ville'                  : faker.address().cityName(),
			'address1'               : faker.address().streetAddress(),
			'address2'               : faker.address().secondaryAddress(),
			'address3'               : faker.address().buildingNumber(),
			'contactInfoEmail'       : faker.internet().emailAddress(),
			'contactInfoPhone'       : faker.phoneNumber().phoneNumber(),
			'contactInfoMobile'      : faker.phoneNumber().cellPhone(),
			'contactInfoFax'         : faker.phoneNumber().phoneNumber(),
			'proprioCompteBancaire'  : faker.name().fullName(),
			'nomDeLaVille'           : faker.address().city(),
			'nomDeLaCopropriete'     : faker.company().name()
		]

		// Fusion avec addMap si override
		Map finalData = localData + addMap

		return finalData
	}
}
