<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DelaisReglement</name>
   <tag></tag>
   <elementGuidId>24ad1d65-b5c6-4235-b49e-78cf031d2a19</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;code\&quot;: \&quot;${CodeCOPRO}\&quot;,\n  \&quot;dueDateDelayEL\&quot;: \&quot;${nombreDeJour}\&quot;,\n  \&quot;customer\&quot;: \&quot;${CodeGEST}\&quot;,\n  \&quot;currency\&quot;: \&quot;EUR\&quot;,\n  \&quot;language\&quot;: \&quot;FRA\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ed4a166b-cfba-4f96-b60d-258cbb6075b9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic b3BlbmNlbGwuYWRtaW46b3BlbmNlbGwuYWRtaW4=</value>
      <webElementGuid>85b95f07-63a3-4ab6-aef3-7c803a0c07eb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL_API}/account/customerAccount/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1cae1e99-56e4-4040-b9a0-cd6b3db14cc1</id>
      <masked>false</masked>
      <name>CodeCOPRO</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>0ce2f6ae-2202-4dd8-95f7-c416fb6212f5</id>
      <masked>false</masked>
      <name>nombreDeJournombreDeJour</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a334aa36-e872-4478-8ee8-ade7f5d0b581</id>
      <masked>false</masked>
      <name>CodeGEST</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
