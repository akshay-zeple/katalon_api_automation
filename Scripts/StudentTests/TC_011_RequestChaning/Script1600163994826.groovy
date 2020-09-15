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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

res1 = WS.sendRequest(findTestObject('StudentsRestAPI/AddNewStudentDynamicdata', [('firstName') : 'Test', ('middleName') : 'Automation'
            , ('lastName') : 'World', ('dateOfBirth') : '9/14/2020\r\n']))

WS.verifyResponseStatusCode(res1, 201)

println(res1.responseText)

def obj = new groovy.json.JsonSlurper()

def jsonData = obj.parseText(res1.responseText)

sID = jsonData.id

res2 = WS.sendRequest(findTestObject('StudentsRestAPI/DeleteStudent', [('studentID') : sID]))

println(res2.responseText)

res3 = WS.sendRequest(findTestObject('StudentsRestAPI/GetStudentDetailsDynamicID', [('studentID') : sID]))

println(res3.responseText)
