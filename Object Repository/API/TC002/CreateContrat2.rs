<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateContrat2</name>
   <tag></tag>
   <elementGuidId>23681d4b-e391-4ced-94d4-d678fc84487d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;userAccount\&quot;: \&quot;${codeCopropriete}\&quot;,\n    \&quot;offerTemplate\&quot;: \&quot;OFFER_Gaz\&quot;,\n    \&quot;renewalRule\&quot;: {\n        \&quot;calendarInitialyActiveFor\&quot;: {}\n    },\n    \&quot;subscriptionDate\&quot;:\&quot;{dateSouscription}\&quot;,\n    \&quot;seller\&quot;: \&quot;AGENCE_Toulouse\&quot;,\n    \&quot;customFields\&quot;: {\n        \&quot;customField\&quot;: [\n            {\n                \&quot;code\&quot;: \&quot;CF_ARRONDI_FACTURE\&quot;,\n                \&quot;longValue\&quot;: 2\n            },\n                      {\n                \&quot;code\&quot;: \&quot;CF_MODE_ARRONDI\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Au supérieur\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_CODE_ANALYTIQUE_AFFAIRE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;999997\&quot;\n            }\n        ]\n    },\n    \&quot;salesPersonName\&quot;: \&quot;\&quot;,\n    \&quot;discountPlanForInstantiation\&quot;: [],\n    \&quot;discountPlanForTermination\&quot;: [],\n    \&quot;priceListCode\&quot;: \&quot;\&quot;,\n    \&quot;paymentMethod\&quot;: null,\n    \&quot;productToInstantiateDto\&quot;: [\n      \t{\n            \&quot;productCode\&quot;: \&quot;PROD_PCMPG\&quot;,\n            \&quot;quantity\&quot;: 10,\n            \&quot;deliveryDate\&quot;: \&quot;${dateDeLivraison}\&quot;,\n            \&quot;attributeInstances\&quot;: [\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Code_Revision\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Nbre_Compteurs\&quot;,\n                    \&quot;doubleValue\&quot;: 8,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n              \t                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Nbre_APP\&quot;,\n                    \&quot;doubleValue\&quot;: 4,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Technologie\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;RADIO\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n              \t{\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Forfaitaire\&quot;,\n                    \&quot;stringValue\&quot;: true,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Revise\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Fluide\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;GAZ\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Libelle\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Initial\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;doubleValue\&quot;: 4.75,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                }\n            ]\n        }\n    ],\n    \&quot;validityDate\&quot;: 1741171420908,\n    \&quot;code\&quot;: \&quot;${codeContrat}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic b3BlbmNlbGwuYWRtaW46b3BlbmNlbGwuYWRtaW4=</value>
      <webElementGuid>2059da4f-486f-4d57-bd35-d7b7a619e6e0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>683f2c59-8e08-4d14-b034-525184df7b06</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL_API}/billing/subscription/subscribeAndInstantiateProducts</restUrl>
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
      <id>46afb700-6cec-4bc7-be7e-37432384e432</id>
      <masked>false</masked>
      <name>codeContrat</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4a34b35b-d9ee-43dc-be3f-eef4df103bff</id>
      <masked>false</masked>
      <name>dateSouscription</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>113f539f-8b74-4334-892d-ea33972d220c</id>
      <masked>false</masked>
      <name>codeCopropriete</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>afed5af2-0d59-4a39-a290-1a7702e9e5e7</id>
      <masked>false</masked>
      <name>dateDeLivraison</name>
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
